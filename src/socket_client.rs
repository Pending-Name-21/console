use std::io::prelude::*;
use std::os::unix::net::UnixStream;

pub fn send_data_to_server(data: &str, socket_path: &str) {
    match UnixStream::connect(socket_path) {
        Ok(mut stream) => {
            stream.write_all(data.as_bytes()).expect("Failed to write to stream");
            println!("Sent data to server: {}", data);
        },
        Err(e) => {
            println!("Failed to connect: {}", e);
        }
    }
}
