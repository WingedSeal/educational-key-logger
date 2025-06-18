use input_linux_sys::*;
use serde::{Deserialize, Serialize};

#[repr(C)]
#[derive(Clone, Copy, Serialize, Deserialize)]
pub struct InputEvent {
    pub time: TimeVal,
    pub event_type: EventType,
    pub code: u16,
    pub value: EventValue,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct TimeVal {
    pub tv_sec: i64,
    pub tv_usec: i64,
}

#[repr(u16)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum EventType {
    /// Synchronization events - used to separate and synchronize events
    Syn = EV_SYN as u16,
    /// Key events - keyboards, buttons, key-like devices (press/release/repeat)
    Key = EV_KEY as u16,
    /// Relative events - mouse movement, scroll wheels (relative changes)
    Rel = EV_REL as u16,
    /// Absolute events - touchpads, joysticks, tablets (absolute positions)
    Abs = EV_ABS as u16,
    /// Miscellaneous events - data that doesn't fit other categories
    Msc = EV_MSC as u16,
    /// Switch events - binary state switches (lid open/closed, etc.)
    Sw = EV_SW as u16,
    /// LED events - control LEDs on devices (Caps Lock, Num Lock, etc.)
    Led = EV_LED as u16,
    /// Sound events - beeps, buzzers, simple audio output
    Snd = EV_SND as u16,
    /// Repeat events - autorepeat configuration for keys
    Rep = EV_REP as u16,
    /// Force feedback events - rumble, vibration effects
    Ff = EV_FF as u16,
    /// Power management events - power button, battery events
    Pwr = EV_PWR as u16,
    /// Force feedback status - feedback from force feedback devices
    FfStatus = EV_FF_STATUS as u16,
}

impl EventType {
    pub fn from_u16(value: u16) -> Option<Self> {
        match value {
            0x00 => Some(EventType::Syn),
            0x01 => Some(EventType::Key),
            0x02 => Some(EventType::Rel),
            0x03 => Some(EventType::Abs),
            0x04 => Some(EventType::Msc),
            0x05 => Some(EventType::Sw),
            0x11 => Some(EventType::Led),
            0x12 => Some(EventType::Snd),
            0x14 => Some(EventType::Rep),
            0x15 => Some(EventType::Ff),
            0x16 => Some(EventType::Pwr),
            0x17 => Some(EventType::FfStatus),
            _ => None,
        }
    }
}

#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct EventValue(pub i32);

impl EventValue {
    pub const RELEASED: EventValue = EventValue(0);
    pub const PRESSED: EventValue = EventValue(1);
    pub const REPEATED: EventValue = EventValue(2);

    pub fn new(value: i32) -> Self {
        EventValue(value)
    }

    pub fn to_i32(&self) -> i32 {
        self.0
    }

    pub fn is_key_released(&self) -> bool {
        *self == Self::RELEASED
    }

    pub fn is_key_pressed(&self) -> bool {
        *self == Self::PRESSED
    }

    pub fn is_key_repeated(&self) -> bool {
        *self == Self::REPEATED
    }

    pub fn to_string(&self) -> Option<&str> {
        match self.0 {
            0 => Some("Released"),
            1 => Some("Pressed"),
            2 => Some("Repeated"),
            _ => None,
        }
    }
}
impl std::fmt::Debug for EventValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.to_string() {
            Some(string) => f.write_str(string),
            _ => f.debug_tuple("EventValue").field(&self.0).finish(),
        }
    }
}
impl InputEvent {
    pub fn is_key_event(&self) -> bool {
        self.event_type == EventType::Key
    }

    pub fn is_key_press(&self) -> bool {
        self.value.is_key_pressed()
    }

    pub fn is_key_release(&self) -> bool {
        self.value.is_key_released()
    }

    pub fn is_sync_event(&self) -> bool {
        self.event_type == EventType::Syn
    }
    pub fn code_as_string(&self) -> &str {
        assert!(
            self.is_key_event(),
            "code_as_string was called on non key event"
        );

        match self.code as i32 {
            KEY_A => "A",
            KEY_B => "B",
            KEY_C => "C",
            KEY_D => "D",
            KEY_E => "E",
            KEY_F => "F",
            KEY_G => "G",
            KEY_H => "H",
            KEY_I => "I",
            KEY_J => "J",
            KEY_K => "K",
            KEY_L => "L",
            KEY_M => "M",
            KEY_N => "N",
            KEY_O => "O",
            KEY_P => "P",
            KEY_Q => "Q",
            KEY_R => "R",
            KEY_S => "S",
            KEY_T => "T",
            KEY_U => "U",
            KEY_V => "V",
            KEY_W => "W",
            KEY_X => "X",
            KEY_Y => "Y",
            KEY_Z => "Z",

            KEY_0 => "0",
            KEY_1 => "1",
            KEY_2 => "2",
            KEY_3 => "3",
            KEY_4 => "4",
            KEY_5 => "5",
            KEY_6 => "6",
            KEY_7 => "7",
            KEY_8 => "8",
            KEY_9 => "9",

            KEY_SPACE => "<Space>",
            KEY_ENTER => "<Enter>",
            KEY_BACKSPACE => "<Backspace>",
            KEY_TAB => "<Tab>",
            KEY_ESC => "<Esc>",

            KEY_MINUS => "-",
            KEY_EQUAL => "=",
            KEY_LEFTBRACE => "[",
            KEY_RIGHTBRACE => "]",
            KEY_SEMICOLON => ";",
            KEY_APOSTROPHE => "'",
            KEY_GRAVE => "`",
            KEY_BACKSLASH => "\\",
            KEY_COMMA => ",",
            KEY_DOT => ".",
            KEY_SLASH => "/",

            KEY_LEFTSHIFT | KEY_RIGHTSHIFT => "<Shift>",
            KEY_LEFTCTRL | KEY_RIGHTCTRL => "<Ctrl>",
            KEY_LEFTALT | KEY_RIGHTALT => "<Alt>",
            KEY_CAPSLOCK => "<CapsLock>",
            KEY_NUMLOCK => "<NumLock>",
            KEY_SCROLLLOCK => "<ScrollLock>",

            KEY_F1 => "<F1>",
            KEY_F2 => "<F2>",
            KEY_F3 => "<F3>",
            KEY_F4 => "<F4>",
            KEY_F5 => "<F5>",
            KEY_F6 => "<F6>",
            KEY_F7 => "<F7>",
            KEY_F8 => "<F8>",
            KEY_F9 => "<F9>",
            KEY_F10 => "<F10>",
            KEY_F11 => "<F11>",
            KEY_F12 => "<F12>",

            KEY_HOME => "<Home>",
            KEY_END => "<End>",
            KEY_PAGEUP => "<PageUp>",
            KEY_PAGEDOWN => "<PageDown>",
            KEY_DELETE => "<Delete>",
            KEY_INSERT => "<Insert>",

            KEY_UP => "<Up Arrow>",
            KEY_DOWN => "<Down Arrow>",
            KEY_LEFT => "<Left Arrow>",
            KEY_RIGHT => "<Right Arrow>",

            KEY_KP0 => "<Numpad 0>",
            KEY_KP1 => "<Numpad 1>",
            KEY_KP2 => "<Numpad 2>",
            KEY_KP3 => "<Numpad 3>",
            KEY_KP4 => "<Numpad 4>",
            KEY_KP5 => "<Numpad 5>",
            KEY_KP6 => "<Numpad 6>",
            KEY_KP7 => "<Numpad 7>",
            KEY_KP8 => "<Numpad 8>",
            KEY_KP9 => "<Numpad 9>",
            KEY_KPMINUS => "<Numpad ->",
            KEY_KPPLUS => "<Numpad +>",
            KEY_KPASTERISK => "<Numpad *>",
            KEY_KPDOT => "<Numpad .>",
            KEY_KPENTER => "<Numpad Enter>",
            KEY_KPSLASH => "<Numpad />",

            KEY_MUTE => "<Mute>",
            KEY_VOLUMEDOWN => "<Volume Down>",
            KEY_VOLUMEUP => "<Volume Up>",

            _ => "?",
        }
    }
}
impl std::fmt::Debug for InputEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let code_debug_string = if self.is_key_event() {
            format!("{} ({:#04x})", self.code_as_string(), self.code)
        } else {
            format!("{:#04x}", self.code)
        };

        f.debug_struct("InputEvent")
            .field("time", &self.time)
            .field("event_type", &self.event_type)
            .field("code", &format_args!("{}", code_debug_string))
            .field("value", &self.value)
            .finish()
    }
}
