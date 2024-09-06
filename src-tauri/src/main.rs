// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::io;
use std::net::{TcpStream, Shutdown};

// Function to connect to a pixel
fn connect_to_pixel(host: &str, port: u16) -> io::Result<TcpStream> {
    let address = format!("{}:{}", host, port);
    let stream = TcpStream::connect(&address)?;
    println!("Connected to {}:{}", host, port);
    Ok(stream)
}

// Function to disconnect from a pixel
fn disconnect_from_pixel(stream: TcpStream) -> io::Result<()> {
    stream.shutdown(Shutdown::Both)?;
    println!("Disconnected from the server.");
    Ok(())
}

#[tauri::command]
fn greet(name: &str) -> String {
    println!("{} has been greeted from Rust!", name);
    format!("Hello, {}! You've been greeted from Rust!", name)
}

// Example Tauri command to connect to the server
#[tauri::command]
fn connect(host: String, port: u16) -> String {
    match connect_to_pixel(&host, port) {
        Ok(_) => format!("Successfully connected to {}:{}", host, port),
        Err(e) => format!("Failed to connect: {}", e),
    }
}

// Example Tauri command to disconnect from the server
#[tauri::command]
fn disconnect(host: String, port: u16) -> String {
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

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet, connect, disconnect])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}