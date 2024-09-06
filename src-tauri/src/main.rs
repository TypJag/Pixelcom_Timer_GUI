// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod tcp; // Include the connect module
mod commands; // Include the commands module

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![commands::greet, commands::connect, commands::disconnect])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}