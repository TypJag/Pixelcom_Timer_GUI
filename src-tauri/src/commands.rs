use crate::tcp::{connect_to_pixel, disconnect_from_pixel, send_to_pixel_time, send_to_pixel_flag_command};

#[tauri::command]
pub fn connect(host: String, port: u16) -> String {
    match connect_to_pixel(&host, port) {
        Ok(_) => format!("Successfully connected to {}:{}", host, port),
        Err(e) => format!("Failed to connect: {}", e),
    }
}

#[tauri::command]
pub fn disconnect() -> String {
    match disconnect_from_pixel() {
        Ok(_) => format!("Successfully disconnected"),
        Err(e) => format!("Failed to disconnect: {}", e),
    }
}

#[tauri::command]
pub fn send_time(time: u32) -> String {
  println!("Sending time to pixel: {}", time);
    match send_to_pixel_time(time) {
        Ok(_) => format!("Successfully sent time to pixel: {}", time),
        Err(e) => format!("Failed to send time to pixel: {}", e),
    }
}
#[tauri::command]
pub fn send_flag(flag: String) -> String {
  println!("Sending flag to pixel: {}", flag);
    match send_to_pixel_flag_command(&flag) {
        Ok(_) => format!("Successfully sent flag to pixel: {}", flag),
        Err(e) => format!("Failed to send flag to pixel: {}", e),
    }
}