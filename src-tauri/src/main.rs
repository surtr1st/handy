#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

#[tauri::command]
fn progress_data() {}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![progress_data])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
