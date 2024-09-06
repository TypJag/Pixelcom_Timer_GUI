use std::io;
use std::net::{TcpStream, Shutdown};

// Function to connect to a pixel
pub fn connect_to_pixel(host: &str, port: u16) -> io::Result<TcpStream> {
    let address = format!("{}:{}", host, port);
    let stream = TcpStream::connect(&address)?;
    println!("Connected to {}:{}", host, port);
    Ok(stream)
}

// Function to disconnect from a pixel
pub fn disconnect_from_pixel(stream: TcpStream) -> io::Result<()> {
    stream.shutdown(Shutdown::Both)?;
    println!("Disconnected from the server.");
    Ok(())
}