use std::sync::atomic::AtomicBool;
use std::sync::Arc;

use tokio::sync::{oneshot, Mutex};

#[derive(Default)]
pub struct AppState {
    pub cancel_flag: Arc<AtomicBool>,
    pub job_id: Mutex<Option<String>>,
    pub kill_tx: Mutex<Option<oneshot::Sender<()>>>,
}

impl AppState {
    pub async fn reset(&self) {
        self.cancel_flag
            .store(false, std::sync::atomic::Ordering::SeqCst);
        *self.job_id.lock().await = None;
        *self.kill_tx.lock().await = None;
    }

    pub async fn begin_job(
        &self,
        job_id: String,
    ) -> Result<oneshot::Receiver<()>, crate::error::AppError> {
        let mut slot = self.kill_tx.lock().await;
        if slot.is_some() {
            return Err(crate::error::AppError::msg("已有转换任务正在进行"));
        }

        self.cancel_flag
            .store(false, std::sync::atomic::Ordering::SeqCst);
        let (tx, rx) = oneshot::channel();
        *slot = Some(tx);
        *self.job_id.lock().await = Some(job_id);
        Ok(rx)
    }
}
