use educational_key_logger::input::InputEvent;
use educational_key_logger::input_event_codes::*;

#[derive(Default, Eq, PartialEq, Clone)]
struct ModifierState {
    shift: bool,
    ctrl: bool,
    alt: bool,
    caps_lock: bool,
    meta: bool,
}

fn key_map(code: u16) -> Option<&'static str> {
    match code {
        KEY_A => Some("a"),
        KEY_B => Some("b"),
        KEY_C => Some("c"),
        KEY_D => Some("d"),
        KEY_E => Some("e"),
        KEY_F => Some("f"),
        KEY_G => Some("g"),
        KEY_H => Some("h"),
        KEY_I => Some("i"),
        KEY_J => Some("j"),
        KEY_K => Some("k"),
        KEY_L => Some("l"),
        KEY_M => Some("m"),
        KEY_N => Some("n"),
        KEY_O => Some("o"),
        KEY_P => Some("p"),
        KEY_Q => Some("q"),
        KEY_R => Some("r"),
        KEY_S => Some("s"),
        KEY_T => Some("t"),
        KEY_U => Some("u"),
        KEY_V => Some("v"),
        KEY_W => Some("w"),
        KEY_X => Some("x"),
        KEY_Y => Some("y"),
        KEY_Z => Some("z"),

        KEY_0 => Some("0"),
        KEY_1 => Some("1"),
        KEY_2 => Some("2"),
        KEY_3 => Some("3"),
        KEY_4 => Some("4"),
        KEY_5 => Some("5"),
        KEY_6 => Some("6"),
        KEY_7 => Some("7"),
        KEY_8 => Some("8"),
        KEY_9 => Some("9"),

        KEY_SPACE => Some(" "),
        KEY_ENTER => Some("<Enter>"),
        KEY_BACKSPACE => Some("<Backspace>"),
        KEY_TAB => Some("<Tab>"),
        KEY_ESC => Some("<Esc>"),

        KEY_MINUS => Some("-"),
        KEY_EQUAL => Some("="),
        KEY_LEFTBRACE => Some("["),
        KEY_RIGHTBRACE => Some("]"),
        KEY_SEMICOLON => Some(";"),
        KEY_APOSTROPHE => Some("'"),
        KEY_GRAVE => Some("`"),
        KEY_BACKSLASH => Some("\\"),
        KEY_COMMA => Some(","),
        KEY_DOT => Some("."),
        KEY_SLASH => Some("/"),

        KEY_LEFTMETA | KEY_RIGHTMETA => Some("<Meta>"),
        KEY_LEFTSHIFT | KEY_RIGHTSHIFT => Some("<Shift>"),
        KEY_LEFTCTRL | KEY_RIGHTCTRL => Some("<Ctrl>"),
        KEY_LEFTALT | KEY_RIGHTALT => Some("<Alt>"),
        KEY_CAPSLOCK => Some("<CapsLock>"),
        KEY_NUMLOCK => Some("<NumLock>"),
        KEY_SCROLLLOCK => Some("<ScrollLock>"),

        KEY_F1 => Some("<F1>"),
        KEY_F2 => Some("<F2>"),
        KEY_F3 => Some("<F3>"),
        KEY_F4 => Some("<F4>"),
        KEY_F5 => Some("<F5>"),
        KEY_F6 => Some("<F6>"),
        KEY_F7 => Some("<F7>"),
        KEY_F8 => Some("<F8>"),
        KEY_F9 => Some("<F9>"),
        KEY_F10 => Some("<F10>"),
        KEY_F11 => Some("<F11>"),
        KEY_F12 => Some("<F12>"),

        KEY_HOME => Some("<Home>"),
        KEY_END => Some("<End>"),
        KEY_PAGEUP => Some("<PageUp>"),
        KEY_PAGEDOWN => Some("<PageDown>"),
        KEY_DELETE => Some("<Delete>"),
        KEY_INSERT => Some("<Insert>"),

        KEY_UP => Some("<UpArrow>"),
        KEY_DOWN => Some("<DownArrow>"),
        KEY_LEFT => Some("<LeftArrow>"),
        KEY_RIGHT => Some("<RightArrow>"),

        KEY_KP0 => Some("<Numpad0>"),
        KEY_KP1 => Some("<Numpad1>"),
        KEY_KP2 => Some("<Numpad2>"),
        KEY_KP3 => Some("<Numpad3>"),
        KEY_KP4 => Some("<Numpad4>"),
        KEY_KP5 => Some("<Numpad5>"),
        KEY_KP6 => Some("<Numpad6>"),
        KEY_KP7 => Some("<Numpad7>"),
        KEY_KP8 => Some("<Numpad8>"),
        KEY_KP9 => Some("<Numpad9>"),
        KEY_KPMINUS => Some("<Numpad->"),
        KEY_KPPLUS => Some("<Numpad+>"),
        KEY_KPASTERISK => Some("<Numpad*>"),
        KEY_KPDOT => Some("<Numpad.>"),
        KEY_KPENTER => Some("<NumpadEnter>"),
        KEY_KPSLASH => Some("<Numpad/>"),

        KEY_MUTE => Some("<Mute>"),
        KEY_VOLUMEDOWN => Some("<VolumeDown>"),
        KEY_VOLUMEUP => Some("<VolumeUp>"),

        KEY_POWER => Some("<Power>"),
        KEY_SYSRQ => Some("<PrintScreen>"),
        KEY_MENU => Some("<Menu>"),
        KEY_PAUSE => Some("<Pause>"),
        _ => None,
    }
}

fn key_map_shift(code: u16) -> Option<&'static str> {
    match code {
        KEY_A => Some("A"),
        KEY_B => Some("B"),
        KEY_C => Some("C"),
        KEY_D => Some("D"),
        KEY_E => Some("E"),
        KEY_F => Some("F"),
        KEY_G => Some("G"),
        KEY_H => Some("H"),
        KEY_I => Some("I"),
        KEY_J => Some("J"),
        KEY_K => Some("K"),
        KEY_L => Some("L"),
        KEY_M => Some("M"),
        KEY_N => Some("N"),
        KEY_O => Some("O"),
        KEY_P => Some("P"),
        KEY_Q => Some("Q"),
        KEY_R => Some("R"),
        KEY_S => Some("S"),
        KEY_T => Some("T"),
        KEY_U => Some("U"),
        KEY_V => Some("V"),
        KEY_W => Some("W"),
        KEY_X => Some("X"),
        KEY_Y => Some("Y"),
        KEY_Z => Some("Z"),

        KEY_0 => Some(")"),
        KEY_1 => Some("!"),
        KEY_2 => Some("@"),
        KEY_3 => Some("#"),
        KEY_4 => Some("$"),
        KEY_5 => Some("%"),
        KEY_6 => Some("^"),
        KEY_7 => Some("&"),
        KEY_8 => Some("*"),
        KEY_9 => Some("("),

        KEY_MINUS => Some("_"),
        KEY_EQUAL => Some("+"),
        KEY_LEFTBRACE => Some("{"),
        KEY_RIGHTBRACE => Some("}"),
        KEY_SEMICOLON => Some(":"),
        KEY_APOSTROPHE => Some("\""),
        KEY_GRAVE => Some("~"),
        KEY_BACKSLASH => Some("|"),
        KEY_COMMA => Some("<"),
        KEY_DOT => Some(">"),
        KEY_SLASH => Some("?"),

        _ => None,
    }
}

pub fn input_events_to_text(events: &[InputEvent]) -> String {
    let mut result = String::with_capacity(events.len() / 2);
    let mut modifier_state = ModifierState::default();
    let mut last_modifier_state = (ModifierState::default(), false);
    for event in events {
        assert!(event.is_key_event());
        if event.is_key_press() {
            handle_key_press(
                event.code,
                &mut modifier_state,
                &mut result,
                &mut last_modifier_state,
            );
        } else if event.is_key_release() {
            handle_key_release(event.code, &mut modifier_state);
        }
    }

    result
}
const BACKSPACE: &str = "\u{0008}";

fn handle_key_press(
    code: u16,
    modifier_state: &mut ModifierState,
    result: &mut String,
    last_modifier_state: &mut (ModifierState, bool),
) {
    match code {
        KEY_LEFTSHIFT | KEY_RIGHTSHIFT => {
            modifier_state.shift = true;
        }
        KEY_LEFTCTRL | KEY_RIGHTCTRL => {
            modifier_state.ctrl = true;
        }
        KEY_LEFTALT | KEY_RIGHTALT => {
            modifier_state.alt = true;
        }
        KEY_LEFTMETA | KEY_RIGHTMETA => {
            modifier_state.meta = true;
        }
        KEY_CAPSLOCK => {
            modifier_state.caps_lock = !modifier_state.caps_lock;
        }
        KEY_BACKSPACE => {
            if modifier_state.ctrl || modifier_state.alt || modifier_state.meta {
                handle_modifier_sequence_char(
                    BACKSPACE,
                    modifier_state,
                    result,
                    true,
                    last_modifier_state,
                );
                last_modifier_state.0 = modifier_state.clone();
                last_modifier_state.1 = true;
            } else {
                if !result.is_empty() && !result.ends_with(">") {
                    result.pop();
                } else {
                    result.push_str("<Backspace>");
                }
            }
        }
        _ => {
            let normal_char = if let Some(normal_char) = key_map(code) {
                normal_char
            } else {
                &format!("<{:#04x}>", code)
            };
            let mut shift_char_not_found = false;
            let should_uppercase = modifier_state.shift ^ modifier_state.caps_lock;
            let ch = if should_uppercase {
                match key_map_shift(code) {
                    Some(shift_char) => shift_char,
                    None => {
                        shift_char_not_found = true;
                        normal_char
                    }
                }
            } else {
                normal_char
            };
            if modifier_state.ctrl
                || modifier_state.alt
                || modifier_state.meta
                || (modifier_state.shift && shift_char_not_found)
            {
                handle_modifier_sequence_char(
                    ch,
                    modifier_state,
                    result,
                    shift_char_not_found,
                    last_modifier_state,
                );
                last_modifier_state.0 = modifier_state.clone();
                last_modifier_state.1 = shift_char_not_found;
            } else {
                result.push_str(ch);
                last_modifier_state.0 = modifier_state.clone();
                last_modifier_state.1 = false;
            }
        }
    }
}
fn handle_key_release(code: u16, modifier_state: &mut ModifierState) {
    match code {
        KEY_LEFTSHIFT | KEY_RIGHTSHIFT => {
            modifier_state.shift = false;
        }
        KEY_LEFTCTRL | KEY_RIGHTCTRL => {
            modifier_state.ctrl = false;
        }
        KEY_LEFTALT | KEY_RIGHTALT => {
            modifier_state.alt = false;
        }
        KEY_LEFTMETA | KEY_RIGHTMETA => {
            modifier_state.meta = false;
        }
        _ => {}
    }
}

fn handle_modifier_sequence_char(
    ch: &str,
    modifier_state: &ModifierState,
    result: &mut String,
    shift_char_not_found: bool,
    last_modifier_state: &mut (ModifierState, bool),
) {
    if result.ends_with('>')
        && last_modifier_state.0 == *modifier_state
        && last_modifier_state.1 == shift_char_not_found
    {
        result.pop();
        append_char_to_sequence(ch, result);
        result.push('>');
        return;
    };
    result.push('<');

    let mut has_modifier = false;
    if modifier_state.meta {
        result.push_str("Meta");
        has_modifier = true;
    }

    if modifier_state.ctrl {
        if has_modifier {
            result.push('-');
        }
        result.push_str("Ctrl");
        has_modifier = true;
    }

    if modifier_state.shift && shift_char_not_found {
        if has_modifier {
            result.push('-');
        }
        result.push_str("Shift");
        has_modifier = true;
    }
    if modifier_state.alt {
        if has_modifier {
            result.push('-');
        }
        result.push_str("Alt");
        has_modifier = true;
    }

    if has_modifier {
        result.push('-');
    }

    append_char_to_sequence(ch, result);
    result.push('>');
}

fn append_char_to_sequence(ch: &str, result: &mut String) {
    if ch == BACKSPACE {
        result.push_str("Backspace");
    } else if ch.starts_with("<") && ch.ends_with(">") {
        result.push_str(&ch[1..ch.len() - 1]);
    } else {
        result.push_str(ch);
    }
}
