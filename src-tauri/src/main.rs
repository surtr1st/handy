#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod progress;
use progress::{select_all, Progress};

#[tauri::command]
fn progress_data() -> String {
    let mut data = String::from("");
    let list = select_all();
    for item in list {
        let Progress { id, name } = item;
        data.push_str(format!("{}, ", name).as_str());
    }
    format!("{}", data).into()
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![progress_data])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
