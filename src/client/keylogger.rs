use educational_key_logger::input::InputEvent;
use std::error::Error;
use std::fs::File;
use std::io::Read;
use std::process::Command;
use std::{fmt, mem};

#[derive(Debug, Clone)]
pub struct InputEventError {
    error_msg: String,
}

impl InputEventError {
    pub fn new(error_msg: String) -> Self {
        InputEventError { error_msg }
    }
}

impl fmt::Display for InputEventError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.error_msg)
    }
}

impl Error for InputEventError {}

pub fn get_key_event(mut device_file: &File, device: &str) -> Result<InputEvent, InputEventError> {
    let mut buf: [u8; 24] = unsafe { mem::zeroed() };
    let num_bytes = device_file.read(&mut buf).map_err(|e| {
        InputEventError::new(format!("Failed to read from device {}: {}", device, e))
    })?;
    if num_bytes != mem::size_of::<InputEvent>() {
        panic!("Error while reading from device file");
    }
    let event: InputEvent = unsafe { mem::transmute(buf) };
    Ok(event)
}

pub fn get_device_file() -> Result<(File, String), InputEventError> {
    let device = get_default_device()?;
    let device_file = File::open(&device).map_err(|e| {
        InputEventError::new(format!("Failed to open device file {}: {}", device, e))
    })?;
    Ok((device_file, device))
}

fn get_default_device() -> Result<String, InputEventError> {
    let mut filenames = get_keyboard_device_filenames()?;
    Ok(filenames.swap_remove(0))
}

fn get_keyboard_device_filenames() -> Result<Vec<String>, InputEventError> {
    let mut command_str = "grep -E 'Handlers|EV' /proc/bus/input/devices".to_string();
    command_str.push_str("| grep -B1 120013");
    command_str.push_str("| grep -Eo event[0-9]+");

    let res = Command::new("sh")
        .arg("-c")
        .arg(command_str)
        .output()
        .map_err(|e| InputEventError::new(format!("Failed to obtain devices list: {}", e)))?;
    let res_str = std::str::from_utf8(&res.stdout)
        .map_err(|e| InputEventError::new(format!("Failed to obtain devices list: {}", e)))?;
    let mut filenames = Vec::new();
    for file in res_str.trim().split('\n') {
        let mut filename = "/dev/input/".to_string();
        filename.push_str(file);
        filenames.push(filename);
    }
    Ok(filenames)
}
