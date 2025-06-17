use educational_key_logger::IP_PORT;
use std::{io::Write, net::TcpStream, thread, time::Duration};

fn main() {
    let mut stream = TcpStream::connect(IP_PORT).unwrap();
    loop {
        if let Err(err) = stream.write(b"Hello chat\n") {
            println!("Failed: {}", err)
        }
        thread::sleep(Duration::from_millis(500));
    }
}
