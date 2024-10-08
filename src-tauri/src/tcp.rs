use std::io::{self, Write};
use std::net::{TcpStream, Shutdown};
use hex;
use lazy_static::lazy_static;
use std::sync::Mutex;

// Define a global TcpStream wrapped in a Mutex
lazy_static! {
    static ref GLOBAL_STREAM: Mutex<Option<TcpStream>> = Mutex::new(None);
}

// Function to connect to a pixel
pub fn connect_to_pixel(host: &str, port: u16) -> io::Result<()> {
    let address = format!("{}:{}", host, port);
    let stream = TcpStream::connect(&address)?;
    println!("Connected to {}:{}", host, port);

    // Lock the Mutex and replace the existing stream
    let mut global_stream = GLOBAL_STREAM.lock().unwrap();
    *global_stream = Some(stream);

    Ok(())
}

// Function to disconnect from a pixel
pub fn disconnect_from_pixel() -> io::Result<()> {
    let mut global_stream = GLOBAL_STREAM.lock().unwrap();
    
    if let Some(stream) = global_stream.take() {
        stream.shutdown(Shutdown::Both)?;
        println!("Disconnected from the server.");
    }

    Ok(())
}

// Function to send time to the pixel
pub fn send_to_pixel_time(time: u32) -> Result<(), String> {
    let mut global_stream = GLOBAL_STREAM.lock().unwrap();
    let connected = global_stream.is_some();

    if connected && time != 0 {
        if let Some(conn) = global_stream.as_mut() {
            // Build the message
            let minutes = time / 60;
            let seconds = time % 60;
            let checksum = 1 + 14 + minutes + seconds;
            let hex_string = format!(
                "6908086900010e{:02X}00{:02X}0000{:02X}16",
                minutes,
                seconds,
                checksum
            );
            
            // Convert the hex string to bytes
            let bytes: Vec<u8> = match hex::decode(&hex_string) {
                Ok(b) => b,
                Err(_) => return Err("Failed to decode hex string".to_string()),
            };

            // Send the bytes
            match conn.write_all(&bytes) {
                Ok(_) => Ok(()),
                Err(_) => Err("Failed to send to pixel".to_string()),
            }
        } else {
            Err("No connection available".to_string())
        }
    } else {
        Ok(())
    }
}

// Function to send flag command to the pixel
pub fn send_to_pixel_flag_command(flag: &str) -> Result<(), String> {
    let connected = GLOBAL_STREAM.lock().unwrap().is_some();
    let hex_string = match flag {
        "no" => "6908086900010000000000000116",     // Turn off penalty board
        "yellow" => "6908086900011c00000000001d16", // Yellow flag
        "finish" => "6908086900010600000000646b16", // Finish flag
        "green" => "6908086900011f00000000002016",  // Green flag (Needs correct value)
        "red" => "6908086900011b00000000001c16",    // Red flag (Needs correct value)
        "showTime" => "6908086900010e09003300004b16", // Show time (Needs correct value)
        _ => return Err("Invalid flag".to_string()),
    };

    if connected {
        // Lock the Mutex and get the global stream
        let mut global_stream = GLOBAL_STREAM.lock().unwrap();

        if let Some(conn) = global_stream.as_mut() {
            // Convert the hex string to bytes
            let bytes: Vec<u8> = match hex::decode(hex_string) {
                Ok(b) => b,
                Err(_) => return Err("Failed to decode hex string".to_string()),
            };

            // Send the bytes
            match conn.write_all(&bytes) {
                Ok(_) => Ok(()),
                Err(_) => Err("Failed to send to pixel".to_string()),
            }
        } else {
            Err("No connection available".to_string())
        }
    } else {
        Ok(())
    }
}