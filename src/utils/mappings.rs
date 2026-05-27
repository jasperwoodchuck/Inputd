use evdev::{
    EventType,
    InputEvent,
    KeyCode,
    RelativeAxisCode,
};

use crate::utils::types::{
    Axis,
    AxisDirection,
    InputToken,
};

#[rustfmt::skip]
pub fn str_to_keycode(key: &str) -> Result<InputToken, String> {
    match key.to_ascii_uppercase().as_str() {
        "A" => Ok(InputToken::Key(KeyCode::KEY_A)),
        "B" => Ok(InputToken::Key(KeyCode::KEY_B)),
        "C" => Ok(InputToken::Key(KeyCode::KEY_C)),
        "D" => Ok(InputToken::Key(KeyCode::KEY_D)),
        "E" => Ok(InputToken::Key(KeyCode::KEY_E)),
        "F" => Ok(InputToken::Key(KeyCode::KEY_F)),
        "G" => Ok(InputToken::Key(KeyCode::KEY_G)),
        "H" => Ok(InputToken::Key(KeyCode::KEY_H)),
        "I" => Ok(InputToken::Key(KeyCode::KEY_I)),
        "J" => Ok(InputToken::Key(KeyCode::KEY_J)),
        "K" => Ok(InputToken::Key(KeyCode::KEY_K)),
        "L" => Ok(InputToken::Key(KeyCode::KEY_L)),
        "M" => Ok(InputToken::Key(KeyCode::KEY_M)),
        "N" => Ok(InputToken::Key(KeyCode::KEY_N)),
        "O" => Ok(InputToken::Key(KeyCode::KEY_O)),
        "P" => Ok(InputToken::Key(KeyCode::KEY_P)),
        "Q" => Ok(InputToken::Key(KeyCode::KEY_Q)),
        "R" => Ok(InputToken::Key(KeyCode::KEY_R)),
        "S" => Ok(InputToken::Key(KeyCode::KEY_S)),
        "T" => Ok(InputToken::Key(KeyCode::KEY_T)),
        "U" => Ok(InputToken::Key(KeyCode::KEY_U)),
        "V" => Ok(InputToken::Key(KeyCode::KEY_V)),
        "W" => Ok(InputToken::Key(KeyCode::KEY_W)),
        "X" => Ok(InputToken::Key(KeyCode::KEY_X)),
        "Y" => Ok(InputToken::Key(KeyCode::KEY_Y)),
        "Z" => Ok(InputToken::Key(KeyCode::KEY_Z)),

        "0" => Ok(InputToken::Key(KeyCode::KEY_0)),
        "1" => Ok(InputToken::Key(KeyCode::KEY_1)),
        "2" => Ok(InputToken::Key(KeyCode::KEY_2)),
        "3" => Ok(InputToken::Key(KeyCode::KEY_3)),
        "4" => Ok(InputToken::Key(KeyCode::KEY_4)),
        "5" => Ok(InputToken::Key(KeyCode::KEY_5)),
        "6" => Ok(InputToken::Key(KeyCode::KEY_6)),
        "7" => Ok(InputToken::Key(KeyCode::KEY_7)),
        "8" => Ok(InputToken::Key(KeyCode::KEY_8)),
        "9" => Ok(InputToken::Key(KeyCode::KEY_9)),

        "SHIFT" | "LSHIFT" => Ok(InputToken::Key(KeyCode::KEY_LEFTSHIFT)),
        "RSHIFT"           => Ok(InputToken::Key(KeyCode::KEY_RIGHTSHIFT)),

        "CTRL" | "LCTRL"   => Ok(InputToken::Key(KeyCode::KEY_LEFTCTRL)),
        "RCTRL"            => Ok(InputToken::Key(KeyCode::KEY_RIGHTCTRL)),

        "ALT" | "LALT"     => Ok(InputToken::Key(KeyCode::KEY_LEFTALT)),
        "RALT"             => Ok(InputToken::Key(KeyCode::KEY_RIGHTALT)),

        "SUPER" | "META"   => Ok(InputToken::Key(KeyCode::KEY_LEFTMETA)),

        "ENTER" | "RETURN" => Ok(InputToken::Key(KeyCode::KEY_ENTER)),
        "SPACE"            => Ok(InputToken::Key(KeyCode::KEY_SPACE)),

        "TAB"              => Ok(InputToken::Key(KeyCode::KEY_TAB)),
        "ESC" | "ESCAPE"   => Ok(InputToken::Key(KeyCode::KEY_ESC)),
        
        "BACKSPACE"        => Ok(InputToken::Key(KeyCode::KEY_BACKSPACE)),
        "CAPSLOCK"         => Ok(InputToken::Key(KeyCode::KEY_CAPSLOCK)),

        "UP"    => Ok(InputToken::Key(KeyCode::KEY_UP)),
        "DOWN"  => Ok(InputToken::Key(KeyCode::KEY_DOWN)),
        
        "LEFT"  => Ok(InputToken::Key(KeyCode::KEY_LEFT)),
        "RIGHT" => Ok(InputToken::Key(KeyCode::KEY_RIGHT)),

        "F1"  => Ok(InputToken::Key(KeyCode::KEY_F1)),
        "F2"  => Ok(InputToken::Key(KeyCode::KEY_F2)),
        "F3"  => Ok(InputToken::Key(KeyCode::KEY_F3)),
        "F4"  => Ok(InputToken::Key(KeyCode::KEY_F4)),
        "F5"  => Ok(InputToken::Key(KeyCode::KEY_F5)),
        "F6"  => Ok(InputToken::Key(KeyCode::KEY_F6)),
        "F7"  => Ok(InputToken::Key(KeyCode::KEY_F7)),
        "F8"  => Ok(InputToken::Key(KeyCode::KEY_F8)),
        "F9"  => Ok(InputToken::Key(KeyCode::KEY_F9)),
        "F10" => Ok(InputToken::Key(KeyCode::KEY_F10)),
        "F11" => Ok(InputToken::Key(KeyCode::KEY_F11)),
        "F12" => Ok(InputToken::Key(KeyCode::KEY_F12)),
        "F13" => Ok(InputToken::Key(KeyCode::KEY_F13)),
        "F14" => Ok(InputToken::Key(KeyCode::KEY_F14)),
        "F15" => Ok(InputToken::Key(KeyCode::KEY_F15)),
        "F16" => Ok(InputToken::Key(KeyCode::KEY_F16)),
        "F17" => Ok(InputToken::Key(KeyCode::KEY_F17)),
        "F18" => Ok(InputToken::Key(KeyCode::KEY_F18)),
        "F19" => Ok(InputToken::Key(KeyCode::KEY_F19)),
        "F20" => Ok(InputToken::Key(KeyCode::KEY_F20)),
        "F21" => Ok(InputToken::Key(KeyCode::KEY_F21)),
        "F22" => Ok(InputToken::Key(KeyCode::KEY_F22)),
        "F23" => Ok(InputToken::Key(KeyCode::KEY_F23)),
        "F24" => Ok(InputToken::Key(KeyCode::KEY_F24)),

        "MOUSE_LEFT"   | "LEFTMOUSE"   => Ok(InputToken::Key(KeyCode::BTN_LEFT)),
        "MOUSE_MIDDLE" | "MIDDLEMOUSE" => Ok(InputToken::Key(KeyCode::BTN_MIDDLE)),
        "MOUSE_RIGHT"  | "RIGHTMOUSE"  => Ok(InputToken::Key(KeyCode::BTN_RIGHT)),

        "WHEEL_UP"    => Ok(InputToken::Scroll(Axis::Y, AxisDirection::Positive)),
        "WHEEL_DOWN"  => Ok(InputToken::Scroll(Axis::Y, AxisDirection::Negative)),
        "WHEEL_RIGHT" => Ok(InputToken::Scroll(Axis::X, AxisDirection::Positive)),
        "WHEEL_LEFT"  => Ok(InputToken::Scroll(Axis::X, AxisDirection::Negative)),

        "MOUSE_SIDE"   | "MOUSE4" => Ok(InputToken::Key(KeyCode::BTN_SIDE)),
        "MOUSE_EXTRA"  | "MOUSE5" => Ok(InputToken::Key(KeyCode::BTN_EXTRA)),

        _ => Err(format!("unknown key: {key}")),
    }
}

#[rustfmt::skip]
pub fn token_to_event(input_token: InputToken, value: i32) -> InputEvent {
    let key_event = EventType::KEY.0;
    let rel_event = EventType::RELATIVE.0;

    match input_token {
        InputToken::Key(code) => InputEvent::new(key_event, code.code(), value),

        InputToken::Scroll(Axis::Y, AxisDirection::Positive) => InputEvent::new(rel_event, RelativeAxisCode::REL_WHEEL.0,  1),
        InputToken::Scroll(Axis::Y, AxisDirection::Negative) => InputEvent::new(rel_event, RelativeAxisCode::REL_WHEEL.0, -1),

        InputToken::Scroll(Axis::X, AxisDirection::Positive) => InputEvent::new(rel_event, RelativeAxisCode::REL_HWHEEL.0,  1),
        InputToken::Scroll(Axis::X, AxisDirection::Negative) => InputEvent::new(rel_event, RelativeAxisCode::REL_HWHEEL.0, -1),
        
        InputToken::MouseMove(Axis::X) => InputEvent::new(rel_event, RelativeAxisCode::REL_X.0, value),
        InputToken::MouseMove(Axis::Y) => InputEvent::new(rel_event, RelativeAxisCode::REL_Y.0, value),
    }
}
