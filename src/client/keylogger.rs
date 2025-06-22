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

/// Get InputEvent (that should be a key event) from device file
///
/// # Arguments
///
/// * `device_file` - A keyboard device file
/// * `device` - Path to device file, used for errors
///
/// # Panics
///
/// * If number of bytes read from device file does not match InputEvent. This should be literally
/// impossible.
///
/// # Errors
///
/// * `InputEventError` - When failing to read from a device file
///
/// # Returns
///
/// An InputEvent that's a key event
pub fn get_key_event(mut device_file: &File, device: &str) -> Result<InputEvent, InputEventError> {
    const INPUT_EVENT_SIZE: usize = mem::size_of::<InputEvent>();
    let mut buf: [u8; INPUT_EVENT_SIZE] = [0; INPUT_EVENT_SIZE];
    let num_bytes = device_file.read(&mut buf).map_err(|e| {
        InputEventError::new(format!("Failed to read from device {}: {}", device, e))
    })?;
    debug_assert!(
        num_bytes == INPUT_EVENT_SIZE,
        "Error while reading from device file"
    );
    let event: InputEvent = unsafe { mem::transmute(buf) };
    Ok(event)
}

/// Find a default device, then get device file from it along with its path
///
/// # Errors
///
/// * `InputEventError` - If device file cannot be openned
///
/// # Returns
///
/// Device file and its path
pub fn get_device_file() -> Result<(File, String), InputEventError> {
    let device = get_default_device()?;
    let device_file = File::open(&device).map_err(|e| {
        InputEventError::new(format!("Failed to open device file {}: {}", device, e))
    })?;
    Ok((device_file, device))
}

/// Get the first device path that's a keyboard
///
/// # Errors
///
/// * `InputEventError` - When failing to obtain devices list
///
/// # Returns
///
/// Path to a device file
fn get_default_device() -> Result<String, InputEventError> {
    let mut filenames = get_keyboard_device_filenames()?;
    Ok(filenames.swap_remove(0))
}

/// Get keyboard device filenames
///
/// Search of device in /procs/bus/input/devices then filter only keyboard devices. Format the
/// devices into device file paths and put them in a vector.
///
/// # Errors
///
/// * `InputEventError` - If executed command to obtain devices fail
///
/// # Returns
///
/// List of all device file paths
fn get_keyboard_device_filenames() -> Result<Vec<String>, InputEventError> {
    const GREP_DEVICES: &str = "grep -E 'Handlers|EV' /proc/bus/input/devices";
    const GREP_KEYBOARDS: &str = "| grep -B1 120013";
    const GREP_NAMES: &str = "| grep -Eo event[0-9]+";
    let mut command_str = GREP_DEVICES.to_string();
    command_str.push_str(GREP_KEYBOARDS);
    command_str.push_str(GREP_NAMES);

    let result = Command::new("sh")
        .arg("-c")
        .arg(command_str)
        .output()
        .map_err(|e| InputEventError::new(format!("Failed to obtain devices list: {}", e)))?;
    let result_string = std::str::from_utf8(&result.stdout)
        .map_err(|e| InputEventError::new(format!("Failed to obtain devices list: {}", e)))?;
    let filenames = result_string
        .trim()
        .split('\n')
        .map(|file| format!("/dev/input/{}", file))
        .collect();
    Ok(filenames)
}
