use serde::{Deserialize, Serialize};

#[repr(C)]
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
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
    Syn = 0x00,
    /// Key events - keyboards, buttons, key-like devices (press/release/repeat)
    Key = 0x01,
    /// Relative events - mouse movement, scroll wheels (relative changes)
    Rel = 0x02,
    /// Absolute events - touchpads, joysticks, tablets (absolute positions)
    Abs = 0x03,
    /// Miscellaneous events - data that doesn't fit other categories
    Msc = 0x04,
    /// Switch events - binary state switches (lid open/closed, etc.)
    Sw = 0x05,
    /// LED events - control LEDs on devices (Caps Lock, Num Lock, etc.)
    Led = 0x11,
    /// Sound events - beeps, buzzers, simple audio output
    Snd = 0x12,
    /// Repeat events - autorepeat configuration for keys
    Rep = 0x14,
    /// Force feedback events - rumble, vibration effects
    Ff = 0x15,
    /// Power management events - power button, battery events
    Pwr = 0x16,
    /// Force feedback status - feedback from force feedback devices
    FfStatus = 0x17,
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
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
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
}
