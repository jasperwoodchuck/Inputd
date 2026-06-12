use evdev::KeyCode;
use phf::phf_map;

use crate::types::input::{
	Axis::{
		X,
		Y,
	},
	Input,
	Magnitude::{
		Negative,
		Positive,
	},
};

#[rustfmt::skip]
pub static INPUTS: phf::Map<&'static str, Input> = phf_map! {
    "A" => Input::Key(KeyCode::KEY_A),
    "B" => Input::Key(KeyCode::KEY_B),
    "C" => Input::Key(KeyCode::KEY_C),
    "D" => Input::Key(KeyCode::KEY_D),
    "E" => Input::Key(KeyCode::KEY_E),
    "F" => Input::Key(KeyCode::KEY_F),
    "G" => Input::Key(KeyCode::KEY_G),
    "H" => Input::Key(KeyCode::KEY_H),
    "I" => Input::Key(KeyCode::KEY_I),
    "J" => Input::Key(KeyCode::KEY_J),
    "K" => Input::Key(KeyCode::KEY_K),
    "L" => Input::Key(KeyCode::KEY_L),
    "M" => Input::Key(KeyCode::KEY_M),
    "N" => Input::Key(KeyCode::KEY_N),
    "O" => Input::Key(KeyCode::KEY_O),
    "P" => Input::Key(KeyCode::KEY_P),
    "Q" => Input::Key(KeyCode::KEY_Q),
    "R" => Input::Key(KeyCode::KEY_R),
    "S" => Input::Key(KeyCode::KEY_S),
    "T" => Input::Key(KeyCode::KEY_T),
    "U" => Input::Key(KeyCode::KEY_U),
    "V" => Input::Key(KeyCode::KEY_V),
    "W" => Input::Key(KeyCode::KEY_W),
    "X" => Input::Key(KeyCode::KEY_X),
    "Y" => Input::Key(KeyCode::KEY_Y),
    "Z" => Input::Key(KeyCode::KEY_Z),

    "0" => Input::Key(KeyCode::KEY_0),
    "1" => Input::Key(KeyCode::KEY_1),
    "2" => Input::Key(KeyCode::KEY_2),
    "3" => Input::Key(KeyCode::KEY_3),
    "4" => Input::Key(KeyCode::KEY_4),
    "5" => Input::Key(KeyCode::KEY_5),
    "6" => Input::Key(KeyCode::KEY_6),
    "7" => Input::Key(KeyCode::KEY_7),
    "8" => Input::Key(KeyCode::KEY_8),
    "9" => Input::Key(KeyCode::KEY_9),

    "MENU"   => Input::Key(KeyCode::KEY_MENU),
    "APPS"   => Input::Key(KeyCode::KEY_MENU),

    "LSHIFT" => Input::Key(KeyCode::KEY_LEFTSHIFT),
    "RSHIFT" => Input::Key(KeyCode::KEY_RIGHTSHIFT),
    "SHIFT"  => Input::Key(KeyCode::KEY_LEFTSHIFT),

    "LCTRL"  => Input::Key(KeyCode::KEY_LEFTCTRL),
    "RCTRL"  => Input::Key(KeyCode::KEY_RIGHTCTRL),
    "CTRL"   => Input::Key(KeyCode::KEY_LEFTCTRL),

    "LALT"   => Input::Key(KeyCode::KEY_LEFTALT),
    "RALT"   => Input::Key(KeyCode::KEY_RIGHTALT),
    "ALT"    => Input::Key(KeyCode::KEY_LEFTALT),

    "LMETA"  => Input::Key(KeyCode::KEY_LEFTMETA),
    "RMETA"  => Input::Key(KeyCode::KEY_RIGHTMETA),
    "META"   => Input::Key(KeyCode::KEY_LEFTMETA),

    "SUPER"  => Input::Key(KeyCode::KEY_LEFTMETA),
    "WIN"    => Input::Key(KeyCode::KEY_LEFTMETA),

    "F1"  => Input::Key(KeyCode::KEY_F1),
    "F2"  => Input::Key(KeyCode::KEY_F2),
    "F3"  => Input::Key(KeyCode::KEY_F3),
    "F4"  => Input::Key(KeyCode::KEY_F4),
    "F5"  => Input::Key(KeyCode::KEY_F5),
    "F6"  => Input::Key(KeyCode::KEY_F6),
    "F7"  => Input::Key(KeyCode::KEY_F7),
    "F8"  => Input::Key(KeyCode::KEY_F8),
    "F9"  => Input::Key(KeyCode::KEY_F9),
    "F10" => Input::Key(KeyCode::KEY_F10),
    "F11" => Input::Key(KeyCode::KEY_F11),
    "F12" => Input::Key(KeyCode::KEY_F12),
    "F13" => Input::Key(KeyCode::KEY_F13),
    "F14" => Input::Key(KeyCode::KEY_F14),
    "F15" => Input::Key(KeyCode::KEY_F15),
    "F16" => Input::Key(KeyCode::KEY_F16),
    "F17" => Input::Key(KeyCode::KEY_F17),
    "F18" => Input::Key(KeyCode::KEY_F18),
    "F19" => Input::Key(KeyCode::KEY_F19),
    "F20" => Input::Key(KeyCode::KEY_F20),
    "F21" => Input::Key(KeyCode::KEY_F21),
    "F22" => Input::Key(KeyCode::KEY_F22),
    "F23" => Input::Key(KeyCode::KEY_F23),
    "F24" => Input::Key(KeyCode::KEY_F24),

    "CAPSLOCK"   => Input::Key(KeyCode::KEY_CAPSLOCK),
    "NUMLOCK"    => Input::Key(KeyCode::KEY_NUMLOCK),
    "SCROLLLOCK" => Input::Key(KeyCode::KEY_SCROLLLOCK),

    "ESC"        => Input::Key(KeyCode::KEY_ESC),
    "ESCAPE"     => Input::Key(KeyCode::KEY_ESC),

    "TAB"        => Input::Key(KeyCode::KEY_TAB),

    "BACKSPACE"  => Input::Key(KeyCode::KEY_BACKSPACE),

    "ENTER"      => Input::Key(KeyCode::KEY_ENTER),
    "RETURN"     => Input::Key(KeyCode::KEY_ENTER),

    "SPACE"      => Input::Key(KeyCode::KEY_SPACE),

    "INSERT"     => Input::Key(KeyCode::KEY_INSERT),

    "DELETE"     => Input::Key(KeyCode::KEY_DELETE),
    "DEL"        => Input::Key(KeyCode::KEY_DELETE),

    "HOME"       => Input::Key(KeyCode::KEY_HOME),
    "END"        => Input::Key(KeyCode::KEY_END),

    "PAGEUP"     => Input::Key(KeyCode::KEY_PAGEUP),
    "PAGEDOWN"   => Input::Key(KeyCode::KEY_PAGEDOWN),

    "PAUSE"      => Input::Key(KeyCode::KEY_PAUSE),
    "PRINT"      => Input::Key(KeyCode::KEY_SYSRQ),

    "UP"         => Input::Key(KeyCode::KEY_UP),
    "RIGHT"      => Input::Key(KeyCode::KEY_RIGHT),
    "DOWN"       => Input::Key(KeyCode::KEY_DOWN),
    "LEFT"       => Input::Key(KeyCode::KEY_LEFT),

    "GRAVE"      => Input::Key(KeyCode::KEY_GRAVE),
    "TILDE"      => Input::Key(KeyCode::KEY_GRAVE),
    "BACKTICK"   => Input::Key(KeyCode::KEY_GRAVE),

    "MINUS"      => Input::Key(KeyCode::KEY_MINUS),
    "EQUAL"      => Input::Key(KeyCode::KEY_EQUAL),

    "LEFTBRACE"  => Input::Key(KeyCode::KEY_LEFTBRACE),
    "RIGHTBRACE" => Input::Key(KeyCode::KEY_RIGHTBRACE),

    "BACKSLASH"  => Input::Key(KeyCode::KEY_BACKSLASH),
    "SEMICOLON"  => Input::Key(KeyCode::KEY_SEMICOLON),

    "APOSTROPHE" => Input::Key(KeyCode::KEY_APOSTROPHE),
    "QUOTE"      => Input::Key(KeyCode::KEY_APOSTROPHE),

    "COMMA"      => Input::Key(KeyCode::KEY_COMMA),

    "DOT"        => Input::Key(KeyCode::KEY_DOT),
    "PERIOD"     => Input::Key(KeyCode::KEY_DOT),

    "SLASH"      => Input::Key(KeyCode::KEY_SLASH),

    "BRIGHTNESS_UP"   => Input::Key(KeyCode::KEY_BRIGHTNESSUP),
    "BRIGHTNESS_DOWN" => Input::Key(KeyCode::KEY_BRIGHTNESSDOWN),

    "POWER"        => Input::Key(KeyCode::KEY_POWER),
    "SLEEP"        => Input::Key(KeyCode::KEY_SLEEP),

    "VOLUME_UP"    => Input::Key(KeyCode::KEY_VOLUMEUP),
    "VOLUME_DOWN"  => Input::Key(KeyCode::KEY_VOLUMEDOWN),
    "MUTE"         => Input::Key(KeyCode::KEY_MUTE),
    "MICMUTE"      => Input::Key(KeyCode::KEY_MICMUTE),

    "WHEEL_UP"     => Input::Wheel(Y, Positive),
    "WHEEL_DOWN"   => Input::Wheel(Y, Negative),
    "WHEEL_RIGHT"  => Input::Wheel(X, Positive),
    "WHEEL_LEFT"   => Input::Wheel(X, Negative),

    "DELTA_UP"     => Input::Delta(Y, Positive),
    "DELTA_DOWN"   => Input::Delta(Y, Negative),
    "DELTA_RIGHT"  => Input::Delta(X, Positive),
    "DELTA_LEFT"   => Input::Delta(X, Negative),

    "MOUSE_LEFT"   => Input::Btn(KeyCode::BTN_LEFT),
    "LEFTMOUSE"    => Input::Btn(KeyCode::BTN_LEFT),

    "MOUSE_MIDDLE" => Input::Btn(KeyCode::BTN_MIDDLE),
    "MIDDLEMOUSE"  => Input::Btn(KeyCode::BTN_MIDDLE),

    "MOUSE_RIGHT"  => Input::Btn(KeyCode::BTN_RIGHT),
    "RIGHTMOUSE"   => Input::Btn(KeyCode::BTN_RIGHT),

    "MOUSE_SIDE"   => Input::Btn(KeyCode::BTN_SIDE),
    "MOUSE4"       => Input::Btn(KeyCode::BTN_SIDE),

    "MOUSE_EXTRA"  => Input::Btn(KeyCode::BTN_EXTRA),
    "MOUSE5"       => Input::Btn(KeyCode::BTN_EXTRA),
};

pub fn str_to_input(key: &str) -> Result<Input, String> {
	INPUTS
		.get(&*key.to_ascii_uppercase())
		.copied()
		.ok_or_else(|| "invalid input".to_string())
}
