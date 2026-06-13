use evdev::{
	EventType,
	InputEvent,
	KeyCode,
	RelativeAxisCode,
};
use phf::phf_map;

use crate::types::input::{
	Axis::{
		X,
		Y,
	},
	InputToken,
	InputValue,
	Magnitude::{
		self,
		Negative,
		Positive,
	},
};

#[inline]
fn input_event_rel(code: u16, value: i32) -> InputEvent {
	InputEvent::new(EventType::RELATIVE.0, code, value)
}

#[rustfmt::skip]
pub fn input_to_event(input_token: &InputToken, input_value: &InputValue) -> InputEvent {
	let value = input_value_to_i32(input_value);

	match input_token {
		InputToken::Key(keycode)  => InputEvent::new(EventType::KEY.0, keycode.code(), value),
		InputToken::Btn(keycode)  => InputEvent::new(EventType::KEY.0, keycode.code(), value),

		InputToken::MouseDelta(Y, _) => input_event_rel(RelativeAxisCode::REL_Y.0, value),
		InputToken::MouseDelta(X, _) => input_event_rel(RelativeAxisCode::REL_X.0, value),

		InputToken::MouseWheel(Y, Positive) => input_event_rel(RelativeAxisCode::REL_WHEEL.0,  1),
		InputToken::MouseWheel(Y, Negative) => input_event_rel(RelativeAxisCode::REL_WHEEL.0, -1),

		InputToken::MouseWheel(X, Positive) => input_event_rel(RelativeAxisCode::REL_HWHEEL.0,  1),
		InputToken::MouseWheel(X, Negative) => input_event_rel(RelativeAxisCode::REL_HWHEEL.0, -1),
	}
}

#[rustfmt::skip]
pub fn magnitude_to_i32(magnitude: &Magnitude) -> i32 {
    match magnitude {
        Positive =>  1,
        Negative => -1,
    }
}

#[rustfmt::skip]
pub fn input_value_to_i32(input_value: &InputValue) -> i32 {
    match input_value {
        InputValue::Press   => 1,
        InputValue::Release => 0,
        InputValue::Repeat  => 2,
        InputValue::Delta(value) => *value,
    }
}
#[rustfmt::skip]
pub static INPUTS: phf::Map<&'static str, InputToken> = phf_map! {
    "A" => InputToken::Key(KeyCode::KEY_A),
    "B" => InputToken::Key(KeyCode::KEY_B),
    "C" => InputToken::Key(KeyCode::KEY_C),
    "D" => InputToken::Key(KeyCode::KEY_D),
    "E" => InputToken::Key(KeyCode::KEY_E),
    "F" => InputToken::Key(KeyCode::KEY_F),
    "G" => InputToken::Key(KeyCode::KEY_G),
    "H" => InputToken::Key(KeyCode::KEY_H),
    "I" => InputToken::Key(KeyCode::KEY_I),
    "J" => InputToken::Key(KeyCode::KEY_J),
    "K" => InputToken::Key(KeyCode::KEY_K),
    "L" => InputToken::Key(KeyCode::KEY_L),
    "M" => InputToken::Key(KeyCode::KEY_M),
    "N" => InputToken::Key(KeyCode::KEY_N),
    "O" => InputToken::Key(KeyCode::KEY_O),
    "P" => InputToken::Key(KeyCode::KEY_P),
    "Q" => InputToken::Key(KeyCode::KEY_Q),
    "R" => InputToken::Key(KeyCode::KEY_R),
    "S" => InputToken::Key(KeyCode::KEY_S),
    "T" => InputToken::Key(KeyCode::KEY_T),
    "U" => InputToken::Key(KeyCode::KEY_U),
    "V" => InputToken::Key(KeyCode::KEY_V),
    "W" => InputToken::Key(KeyCode::KEY_W),
    "X" => InputToken::Key(KeyCode::KEY_X),
    "Y" => InputToken::Key(KeyCode::KEY_Y),
    "Z" => InputToken::Key(KeyCode::KEY_Z),

    "0" => InputToken::Key(KeyCode::KEY_0),
    "1" => InputToken::Key(KeyCode::KEY_1),
    "2" => InputToken::Key(KeyCode::KEY_2),
    "3" => InputToken::Key(KeyCode::KEY_3),
    "4" => InputToken::Key(KeyCode::KEY_4),
    "5" => InputToken::Key(KeyCode::KEY_5),
    "6" => InputToken::Key(KeyCode::KEY_6),
    "7" => InputToken::Key(KeyCode::KEY_7),
    "8" => InputToken::Key(KeyCode::KEY_8),
    "9" => InputToken::Key(KeyCode::KEY_9),

    "MENU"   => InputToken::Key(KeyCode::KEY_MENU),
    "APPS"   => InputToken::Key(KeyCode::KEY_MENU),

    "LSHIFT" => InputToken::Key(KeyCode::KEY_LEFTSHIFT),
    "RSHIFT" => InputToken::Key(KeyCode::KEY_RIGHTSHIFT),
    "SHIFT"  => InputToken::Key(KeyCode::KEY_LEFTSHIFT),

    "LCTRL"  => InputToken::Key(KeyCode::KEY_LEFTCTRL),
    "RCTRL"  => InputToken::Key(KeyCode::KEY_RIGHTCTRL),
    "CTRL"   => InputToken::Key(KeyCode::KEY_LEFTCTRL),

    "LALT"   => InputToken::Key(KeyCode::KEY_LEFTALT),
    "RALT"   => InputToken::Key(KeyCode::KEY_RIGHTALT),
    "ALT"    => InputToken::Key(KeyCode::KEY_LEFTALT),

    "LMETA"  => InputToken::Key(KeyCode::KEY_LEFTMETA),
    "RMETA"  => InputToken::Key(KeyCode::KEY_RIGHTMETA),
    "META"   => InputToken::Key(KeyCode::KEY_LEFTMETA),

    "SUPER"  => InputToken::Key(KeyCode::KEY_LEFTMETA),
    "WIN"    => InputToken::Key(KeyCode::KEY_LEFTMETA),

    "F1"  => InputToken::Key(KeyCode::KEY_F1),
    "F2"  => InputToken::Key(KeyCode::KEY_F2),
    "F3"  => InputToken::Key(KeyCode::KEY_F3),
    "F4"  => InputToken::Key(KeyCode::KEY_F4),
    "F5"  => InputToken::Key(KeyCode::KEY_F5),
    "F6"  => InputToken::Key(KeyCode::KEY_F6),
    "F7"  => InputToken::Key(KeyCode::KEY_F7),
    "F8"  => InputToken::Key(KeyCode::KEY_F8),
    "F9"  => InputToken::Key(KeyCode::KEY_F9),
    "F10" => InputToken::Key(KeyCode::KEY_F10),
    "F11" => InputToken::Key(KeyCode::KEY_F11),
    "F12" => InputToken::Key(KeyCode::KEY_F12),
    "F13" => InputToken::Key(KeyCode::KEY_F13),
    "F14" => InputToken::Key(KeyCode::KEY_F14),
    "F15" => InputToken::Key(KeyCode::KEY_F15),
    "F16" => InputToken::Key(KeyCode::KEY_F16),
    "F17" => InputToken::Key(KeyCode::KEY_F17),
    "F18" => InputToken::Key(KeyCode::KEY_F18),
    "F19" => InputToken::Key(KeyCode::KEY_F19),
    "F20" => InputToken::Key(KeyCode::KEY_F20),
    "F21" => InputToken::Key(KeyCode::KEY_F21),
    "F22" => InputToken::Key(KeyCode::KEY_F22),
    "F23" => InputToken::Key(KeyCode::KEY_F23),
    "F24" => InputToken::Key(KeyCode::KEY_F24),

    "CAPSLOCK"   => InputToken::Key(KeyCode::KEY_CAPSLOCK),
    "NUMLOCK"    => InputToken::Key(KeyCode::KEY_NUMLOCK),
    "SCROLLLOCK" => InputToken::Key(KeyCode::KEY_SCROLLLOCK),

    "ESC"        => InputToken::Key(KeyCode::KEY_ESC),
    "ESCAPE"     => InputToken::Key(KeyCode::KEY_ESC),

    "TAB"        => InputToken::Key(KeyCode::KEY_TAB),

    "BACKSPACE"  => InputToken::Key(KeyCode::KEY_BACKSPACE),

    "ENTER"      => InputToken::Key(KeyCode::KEY_ENTER),
    "RETURN"     => InputToken::Key(KeyCode::KEY_ENTER),

    "SPACE"      => InputToken::Key(KeyCode::KEY_SPACE),

    "INSERT"     => InputToken::Key(KeyCode::KEY_INSERT),

    "DELETE"     => InputToken::Key(KeyCode::KEY_DELETE),
    "DEL"        => InputToken::Key(KeyCode::KEY_DELETE),

    "HOME"       => InputToken::Key(KeyCode::KEY_HOME),
    "END"        => InputToken::Key(KeyCode::KEY_END),

    "PAGEUP"     => InputToken::Key(KeyCode::KEY_PAGEUP),
    "PAGEDOWN"   => InputToken::Key(KeyCode::KEY_PAGEDOWN),

    "PAUSE"      => InputToken::Key(KeyCode::KEY_PAUSE),
    "PRINT"      => InputToken::Key(KeyCode::KEY_SYSRQ),

    "UP"         => InputToken::Key(KeyCode::KEY_UP),
    "RIGHT"      => InputToken::Key(KeyCode::KEY_RIGHT),
    "DOWN"       => InputToken::Key(KeyCode::KEY_DOWN),
    "LEFT"       => InputToken::Key(KeyCode::KEY_LEFT),

    "GRAVE"      => InputToken::Key(KeyCode::KEY_GRAVE),
    "TILDE"      => InputToken::Key(KeyCode::KEY_GRAVE),
    "BACKTICK"   => InputToken::Key(KeyCode::KEY_GRAVE),

    "MINUS"      => InputToken::Key(KeyCode::KEY_MINUS),
    "EQUAL"      => InputToken::Key(KeyCode::KEY_EQUAL),

    "LEFTBRACE"  => InputToken::Key(KeyCode::KEY_LEFTBRACE),
    "RIGHTBRACE" => InputToken::Key(KeyCode::KEY_RIGHTBRACE),

    "BACKSLASH"  => InputToken::Key(KeyCode::KEY_BACKSLASH),
    "SEMICOLON"  => InputToken::Key(KeyCode::KEY_SEMICOLON),

    "APOSTROPHE" => InputToken::Key(KeyCode::KEY_APOSTROPHE),
    "QUOTE"      => InputToken::Key(KeyCode::KEY_APOSTROPHE),

    "COMMA"      => InputToken::Key(KeyCode::KEY_COMMA),

    "DOT"        => InputToken::Key(KeyCode::KEY_DOT),
    "PERIOD"     => InputToken::Key(KeyCode::KEY_DOT),

    "SLASH"      => InputToken::Key(KeyCode::KEY_SLASH),

    "BRIGHTNESS_UP"   => InputToken::Key(KeyCode::KEY_BRIGHTNESSUP),
    "BRIGHTNESS_DOWN" => InputToken::Key(KeyCode::KEY_BRIGHTNESSDOWN),

    "POWER"        => InputToken::Key(KeyCode::KEY_POWER),
    "SLEEP"        => InputToken::Key(KeyCode::KEY_SLEEP),

    "VOLUME_UP"    => InputToken::Key(KeyCode::KEY_VOLUMEUP),
    "VOLUME_DOWN"  => InputToken::Key(KeyCode::KEY_VOLUMEDOWN),
    "MUTE"         => InputToken::Key(KeyCode::KEY_MUTE),
    "MICMUTE"      => InputToken::Key(KeyCode::KEY_MICMUTE),

    "WHEEL_UP"     => InputToken::MouseWheel(Y, Positive),
    "WHEEL_DOWN"   => InputToken::MouseWheel(Y, Negative),
    "WHEEL_RIGHT"  => InputToken::MouseWheel(X, Positive),
    "WHEEL_LEFT"   => InputToken::MouseWheel(X, Negative),

    "DELTA_UP"     => InputToken::MouseDelta(Y, Positive),
    "DELTA_DOWN"   => InputToken::MouseDelta(Y, Negative),
    "DELTA_RIGHT"  => InputToken::MouseDelta(X, Positive),
    "DELTA_LEFT"   => InputToken::MouseDelta(X, Negative),

    "MOUSE_LEFT"   => InputToken::Btn(KeyCode::BTN_LEFT),
    "LEFTMOUSE"    => InputToken::Btn(KeyCode::BTN_LEFT),

    "MOUSE_MIDDLE" => InputToken::Btn(KeyCode::BTN_MIDDLE),
    "MIDDLEMOUSE"  => InputToken::Btn(KeyCode::BTN_MIDDLE),

    "MOUSE_RIGHT"  => InputToken::Btn(KeyCode::BTN_RIGHT),
    "RIGHTMOUSE"   => InputToken::Btn(KeyCode::BTN_RIGHT),

    "MOUSE_SIDE"   => InputToken::Btn(KeyCode::BTN_SIDE),
    "MOUSE4"       => InputToken::Btn(KeyCode::BTN_SIDE),

    "MOUSE_EXTRA"  => InputToken::Btn(KeyCode::BTN_EXTRA),
    "MOUSE5"       => InputToken::Btn(KeyCode::BTN_EXTRA),
};

pub fn str_to_input_token(key: &str) -> Result<InputToken, String> {
	INPUTS
		.get(&*key.to_ascii_uppercase())
		.copied()
		.ok_or_else(|| "invalid input".to_string())
}
