use crate::input_event_codes::*;
use serde::{Deserialize, Serialize};

/// InputEvent in linux kernal
/// - [The Linux Kernal 1.5. Event
/// Interface](https://www.kernel.org/doc/html/latest/input/input.html#event-interface)
/// - [The Linux Kernal 2. Input event codes](https://www.kernel.org/doc/html/latest/input/event-codes.html#input-event-codes)
/// - [`input-event-code.h`](https://github.com/torvalds/linux/blob/master/include/uapi/linux/input-event-codes.h)
#[repr(C)]
#[derive(Clone, Copy, Serialize, Deserialize)]
pub struct InputEvent {
    /// The timestamp, it returns the time at which the event happened.
    /// Type is for example EV_REL for relative movement, EV_KEY for a keypress or release.
    pub time: TimeVal,
    /// - [The Linux Kernal 2.1 Event types](https://www.kernel.org/doc/html/latest/input/event-codes.html#event-types)
    pub event_type: EventType,
    /// - [The Linux Kernal 2.2 Event codes](https://www.kernel.org/doc/html/latest/input/event-codes.html#event-codes)
    pub code: u16,
    /// The value the event carries. Either a relative change for EV_REL, absolute new value for EV_ABS (joysticks ...),
    /// or 0 for EV_KEY for release, 1 for keypress and 2 for autorepeat.
    pub value: EventValue,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct TimeVal {
    pub tv_sec: i64,
    pub tv_usec: i64,
}

/// - [The Linux Kernal 2.1 Event types](https://www.kernel.org/doc/html/latest/input/event-codes.html#event-types)
#[repr(u16)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum EventType {
    /// Synchronization events - Used as markers to separate events. Events may be separated in time or in space, such as with the multitouch protocol.
    Syn = EV_SYN,
    /// Key events - Used to describe state changes of keyboards, buttons, or other key-like devices.
    Key = EV_KEY,
    /// Relative events - Used to describe relative axis value changes, e.g. moving the mouse 5 units to the left.
    Rel = EV_REL,
    /// Absolute events - Used to describe absolute axis value changes, e.g. describing the coordinates of a touch on a touchscreen.
    Abs = EV_ABS,
    /// Miscellaneous events - Used to describe miscellaneous input data that do not fit into other types.
    Msc = EV_MSC,
    /// Switch events - Used to describe binary state input switches.
    Sw = EV_SW,
    /// LED events - Used to turn LEDs on devices on and off.
    Led = EV_LED,
    /// Sound events - Used to output sound to devices.
    Snd = EV_SND,
    /// Repeat events - Used for autorepeating devices.
    Rep = EV_REP,
    /// Force feedback events - Used to send force feedback commands to an input device.
    Ff = EV_FF,
    /// Power management events - A special type for power button and switch input.
    Pwr = EV_PWR,
    /// Force feedback status - Used to receive force feedback device status.
    FfStatus = EV_FF_STATUS,
}

/// The value the event carries. Either a relative change for EV_REL, absolute new value for EV_ABS (joysticks ...),
/// or 0 for EV_KEY for release, 1 for keypress and 2 for autorepeat.
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
            None => f.debug_tuple("EventValue").field(&self.0).finish(),
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

    /// Convert a single InputEvent into a string. Used mainly for debugging.
    pub fn code_as_string(&self) -> &str {
        assert!(
            self.is_key_event(),
            "code_as_string was called on non key event"
        );

        match self.code {
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

            KEY_SPACE => " ",
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

            KEY_LEFTMETA | KEY_RIGHTMETA => "<Meta>",
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

            KEY_POWER => "<Power>",
            KEY_SYSRQ => "<Print Screen>",
            KEY_MENU => "<Menu>",
            KEY_PAUSE => "<Pause>",

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
