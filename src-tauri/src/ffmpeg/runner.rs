use std::process::Stdio;
use std::sync::atomic::Ordering;
use std::time::Duration;

use tauri::{AppHandle, Emitter};
use tokio::io::{AsyncBufReadExt, BufReader};
use tokio::process::Command;
use tokio::sync::oneshot;

use crate::error::AppError;
use crate::ffmpeg::args::build_args;
use crate::ffmpeg::binary::find_ffmpeg;
use crate::models::{ConvertOutcome, ConvertProgress, ConvertRequest};
use crate::state::AppState;

const PROGRESS_EVENT: &str = "convert-progress";

pub async fn run_convert(
    app: AppHandle,
    state: &AppState,
    request: ConvertRequest,
    duration_secs: f64,
) -> Result<ConvertOutcome, AppError> {
    let ffmpeg = find_ffmpeg()?;
    let args = build_args(&request)?;
    let kill_rx = state.begin_job(request.job_id.clone()).await?;

    let mut command = Command::new(&ffmpeg);
    command
        .args(&args)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .kill_on_drop(true);

    #[cfg(windows)]
    {
        command.creation_flags(0x08000000);
    }

    let mut child = match command.spawn() {
        Ok(child) => child,
        Err(err) => {
            state.reset().await;
            return Err(err.into());
        }
    };

    let stdout = child.stdout.take();
    let stderr = child.stderr.take();

    emit_progress(
        &app,
        ConvertProgress {
            job_id: request.job_id.clone(),
            ratio: 0.0,
            out_time_secs: 0.0,
            speed: None,
            stage: "running".into(),
            message: Some("正在转换…".into()),
        },
    );

    let job_id = request.job_id.clone();
    let app_progress = app.clone();
    let progress_task = tokio::spawn(async move {
        if let Some(stdout) = stdout {
            parse_progress(stdout, app_progress, job_id, duration_secs).await;
        }
    });

    let stderr_task = tokio::spawn(async move {
        let mut collected = String::new();
        if let Some(stderr) = stderr {
            let mut lines = BufReader::new(stderr).lines();
            while let Ok(Some(line)) = lines.next_line().await {
                collected.push_str(&line);
                collected.push('\n');
            }
        }
        collected
    });

    let wait_result = wait_or_cancel(&mut child, kill_rx).await;
    let _ = progress_task.await;
    let stderr_output = stderr_task.await.unwrap_or_default();
    let cancelled = state.cancel_flag.load(Ordering::SeqCst);
    state.reset().await;

    if cancelled {
        emit_progress(
            &app,
            ConvertProgress {
                job_id: request.job_id.clone(),
                ratio: 0.0,
                out_time_secs: 0.0,
                speed: None,
                stage: "cancelled".into(),
                message: Some("已取消转换".into()),
            },
        );
        return Ok(ConvertOutcome {
            status: "cancelled".into(),
            output_path: None,
        });
    }

    let status = wait_result?;

    if !status.success() {
        let detail = stderr_output
            .trim()
            .lines()
            .rev()
            .find(|line| !line.is_empty())
            .unwrap_or("FFmpeg 转换失败");
        emit_progress(
            &app,
            ConvertProgress {
                job_id: request.job_id.clone(),
                ratio: 0.0,
                out_time_secs: 0.0,
                speed: None,
                stage: "error".into(),
                message: Some(detail.to_string()),
            },
        );
        return Err(AppError::msg(detail));
    }

    emit_progress(
        &app,
        ConvertProgress {
            job_id: request.job_id,
            ratio: 1.0,
            out_time_secs: duration_secs,
            speed: None,
            stage: "done".into(),
            message: Some("转换完成".into()),
        },
    );

    Ok(ConvertOutcome {
        status: "done".into(),
        output_path: Some(request.output_path),
    })
}

pub async fn cancel_running(state: &AppState) -> Result<(), AppError> {
    state.cancel_flag.store(true, Ordering::SeqCst);
    if let Some(tx) = state.kill_tx.lock().await.take() {
        let _ = tx.send(());
    }
    Ok(())
}

async fn wait_or_cancel(
    child: &mut tokio::process::Child,
    kill_rx: oneshot::Receiver<()>,
) -> Result<std::process::ExitStatus, AppError> {
    tokio::select! {
        status = child.wait() => Ok(status?),
        _ = kill_rx => {
            let _ = child.start_kill();
            let status = tokio::time::timeout(Duration::from_secs(8), child.wait()).await;
            match status {
                Ok(Ok(code)) => Ok(code),
                _ => Err(AppError::Cancelled),
            }
        }
    }
}

async fn parse_progress<R>(stdout: R, app: AppHandle, job_id: String, duration_secs: f64)
where
    R: tokio::io::AsyncRead + Unpin,
{
    let mut lines = BufReader::new(stdout).lines();
    let mut out_time_secs = 0.0;
    let mut speed = None;

    while let Ok(Some(line)) = lines.next_line().await {
        let Some((key, value)) = line.split_once('=') else {
            continue;
        };

        match key {
            "out_time_us" => {
                if let Ok(us) = value.trim().parse::<f64>() {
                    out_time_secs = us / 1_000_000.0;
                }
            }
            "out_time_ms" => {
                if let Ok(ms) = value.trim().parse::<f64>() {
                    out_time_secs = ms / 1_000.0;
                }
            }
            "speed" => {
                let trimmed = value.trim();
                if trimmed != "N/A" && !trimmed.is_empty() {
                    speed = Some(trimmed.to_string());
                }
            }
            "progress" => {
                let ratio = if duration_secs > 0.0 {
                    (out_time_secs / duration_secs).clamp(0.0, 0.99)
                } else {
                    0.0
                };
                let done = value.trim() == "end";
                emit_progress(
                    &app,
                    ConvertProgress {
                        job_id: job_id.clone(),
                        ratio: if done { 1.0 } else { ratio },
                        out_time_secs,
                        speed: speed.clone(),
                        stage: if done {
                            "done".into()
                        } else {
                            "running".into()
                        },
                        message: None,
                    },
                );
            }
            _ => {}
        }
    }
}

fn emit_progress(app: &AppHandle, payload: ConvertProgress) {
    let _ = app.emit(PROGRESS_EVENT, payload);
}
