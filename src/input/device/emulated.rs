use std::io;

use evdev::{
	AttributeSet,
	KeyCode,
	RelativeAxisCode,
	uinput::VirtualDevice,
};

use crate::{
	input::mapping::INPUTS,
	types::input::InputToken,
};

pub fn virtual_keyboard() -> io::Result<VirtualDevice> {
	let mut keys = AttributeSet::<KeyCode>::new();

	for input in INPUTS.values() {
		if let InputToken::Key(key) = input {
			keys.insert(*key);
		}
	}

	VirtualDevice::builder()?
		.name("inputd virtual keyboard")
		.with_keys(&keys)?
		.build()
}

pub fn virtual_mousedev() -> io::Result<VirtualDevice> {
	let mut btns = AttributeSet::<KeyCode>::new();

	for btn in [
		KeyCode::BTN_LEFT,
		KeyCode::BTN_MIDDLE,
		KeyCode::BTN_RIGHT,
		KeyCode::BTN_SIDE,
		KeyCode::BTN_EXTRA,
	] {
		btns.insert(btn);
	}

	let mut axes = AttributeSet::<RelativeAxisCode>::new();

	for axis in [
		RelativeAxisCode::REL_X,
		RelativeAxisCode::REL_Y,
		RelativeAxisCode::REL_WHEEL,
		RelativeAxisCode::REL_HWHEEL,
	] {
		axes.insert(axis);
	}

	VirtualDevice::builder()?
		.name("inputd virtual mouse")
		.with_keys(&btns)?
		.with_relative_axes(&axes)?
		.build()
}
