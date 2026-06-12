use std::{
	sync::mpsc,
	thread,
};

use evdev::Device;

use crate::{
	input::device::physical::{
		keyboard::read_physical_keyboard,
		mousedev::read_physical_mousedev,
	},
	types::input::InputMessage,
};

pub fn start(keyboard: Device, mousedev: Device) {
	let (tx, rx) = mpsc::channel::<InputMessage>();

	thread::spawn({
		let tx = tx.clone();
		move || read_physical_keyboard(tx, keyboard)
	});

	thread::spawn({
		let tx = tx.clone();
		move || read_physical_mousedev(tx, mousedev)
	});

	loop {
		let input_message = rx.recv().expect("recv error");

		println!("{input_message:?}");
	}
}
