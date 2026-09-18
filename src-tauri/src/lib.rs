// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
use chrono::{Local};

#[tauri::command]

fn greet(name: &str) -> String {

    let local: &str = &Local::now().format("%H:%M:%S").to_string();
    format!("Hello, {}! You've been greeted from Rust! at {}!", name, local)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
