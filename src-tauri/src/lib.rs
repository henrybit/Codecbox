mod commands;
mod error;
mod ffmpeg;
mod models;
mod state;

use state::AppState;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
            if cfg!(debug_assertions) {
                app.handle().plugin(
                    tauri_plugin_log::Builder::default()
                        .level(log::LevelFilter::Info)
                        .build(),
                )?;
            }
            Ok(())
        })
        .manage(AppState::default())
        .invoke_handler(tauri::generate_handler![
            commands::check_ffmpeg,
            commands::list_formats,
            commands::probe_media,
            commands::suggest_output_path,
            commands::start_convert,
            commands::cancel_convert,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
