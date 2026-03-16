#![warn(clippy::pedantic)]

mod adb;
mod entries;
mod mix_media;

#[tauri::command]
fn get_project_version() -> &'static str {
    env!("PROJECT_VERSION")
}

/// Run tauri ui
/// # Panics
/// Panics if tauri app faild to run
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_dialog::init())
        .invoke_handler(tauri::generate_handler![
            adb::get_devices,
            adb::get_all_pages,
            entries::probe_entry,
            entries::pull_media,
            get_project_version,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
