#[derive(Debug)]
#[repr(C)]
pub struct InputEvent {
    tv_sec: isize,  // from timeval struct
    tv_usec: isize, // from timeval struct
    pub type_: u16,
    pub code: u16,
    pub value: i32,
}
