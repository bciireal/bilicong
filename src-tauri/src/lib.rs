#![warn(clippy::pedantic)]

mod adb;
mod entries;
mod mix_media;

fn log_init() {
    use tracing_subscriber::filter::{EnvFilter, LevelFilter};

    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::builder()
                .with_default_directive(LevelFilter::WARN.into())
                .from_env_lossy(),
        )
        .init();
}

/// Run tauri ui
/// # Panics
/// Panics if tauri app faild to run
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_dialog::init())
        .setup(|_| {
            if cfg!(debug_assertions) {
                log_init();
            }
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            adb::get_devices,
            adb::get_all_pages,
            entries::probe_entry,
            entries::pull_media,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
