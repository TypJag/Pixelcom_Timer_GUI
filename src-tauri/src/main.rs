// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod tcp; // Include the connect module
mod commands; // Include the commands module

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![commands::connect, commands::disconnect, commands::send_time, commands::send_flag])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}