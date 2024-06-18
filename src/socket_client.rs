use std::io::prelude::*;
use std::net::TcpStream;

pub fn send_data_to_server(data: &str, server_address: &str) {
    match TcpStream::connect(server_address) {
        Ok(mut stream) => {
            stream.write_all(data.as_bytes()).expect("Failed to write to stream");
            println!("Sent data to server: {}", data);
        },
        Err(e) => {
            println!("Failed to connect: {}", e);
        }
    }
}
