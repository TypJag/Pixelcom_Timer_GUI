use crate::tcp::{connect_to_pixel, disconnect_from_pixel};
use std::net::TcpStream;

#[tauri::command]
pub fn greet(name: &str) -> String {
    println!("{} has been greeted from Rust!", name);
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
pub fn connect(host: String, port: u16) -> String {
    match connect_to_pixel(&host, port) {
        Ok(_) => format!("Successfully connected to {}:{}", host, port),
        Err(e) => format!("Failed to connect: {}", e),
    }
}

#[tauri::command]
pub fn disconnect(host: String, port: u16) -> String {
    match connect_to_pixel(&host, port) {
        Ok(stream) => {
            if let Err(e) = disconnect_from_pixel(stream) {
                return format!("Failed to disconnect: {}", e);
            }
            format!("Successfully disconnected from {}:{}", host, port)
        }
        Err(e) => format!("Failed to connect for disconnecting: {}", e),
    }
}