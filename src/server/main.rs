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
    let mut stream_read_handler = StreamReadHandler::new();
    loop {
        match stream.read(&mut buffer) {
            Ok(0) => {
                println!("Client disconnected");
                break;
            }
            Ok(bytes_read) => {
                stream_read_handler.read_buffer(&buffer[..bytes_read]);
            }
            Err(e) => {
                println!("Read error: {}", e);
                break;
            }
        }
    }
}

struct StreamReadHandler {
    size_prefix: Option<usize>,
    /// Buffer for InputEvent. It's length can never exceed size_prefix which is sent through TCPStream.
    input_event_buffer: Vec<u8>,
}

impl StreamReadHandler {
    pub fn new() -> StreamReadHandler {
        StreamReadHandler {
            size_prefix: None,
            input_event_buffer: vec![],
        }
    }
    pub fn read_buffer(&mut self, buffer: &[u8]) {
        let mut buffer_index = 0;
        while buffer_index < buffer.len() {
            match self.size_prefix {
                None => {
                    self.size_prefix = Some(buffer[buffer_index].into());
                    // There'll be at least 1 byte in the buffer. Since bytes_read cannot be 0.
                    debug_assert!(
                        self.input_event_buffer.len() == 0,
                        "input_event_buffer should be cleared."
                    );
                    buffer_index += 1;
                }
                Some(size_prefix) => {
                    let available_space = size_prefix - self.input_event_buffer.len();
                    let will_buffer_be_full = buffer.len() - buffer_index >= available_space;
                    if will_buffer_be_full {
                        self.input_event_buffer.extend_from_slice(
                            &buffer[buffer_index..(buffer_index + available_space)],
                        );
                        buffer_index += available_space;
                        self.input_event_buffer_full();
                    } else {
                        self.input_event_buffer
                            .extend_from_slice(&buffer[buffer_index..]);
                    }
                }
            };
        }
    }
    fn input_event_buffer_full(&mut self) {
        let input_event: InputEvent = postcard::from_bytes(&self.input_event_buffer)
            .expect("Handler ensures complete and correctly framed InputEvent");
        self.input_event_buffer.clear();
        self.size_prefix = None;
        handle_input_event(input_event);
    }
}

fn handle_input_event(input_event: InputEvent) {
    dbg!(input_event);
}
