use std::{
	sync::mpsc,
	thread,
};

use evdev::{
	Device,
	uinput::VirtualDevice,
};

use crate::{
	input::{
		device::physical::{
			keyboard::read_physical_keyboard,
			mousedev::read_physical_mousedev,
		},
		rebind::runtime::Runtime,
	},
	types::input::{
		InputMessage,
		InputToken,
		RebindDict,
	},
};

pub fn start(
	rebind_dict: RebindDict,
	virtual_keyboard: VirtualDevice,
	virtual_mousedev: VirtualDevice,
	keyboard: Device,
	mousedev: Device,
) {
	let (tx, rx) = mpsc::channel::<InputMessage>();

	let mut runtime = Runtime::new(rebind_dict, virtual_keyboard, virtual_mousedev);

	thread::spawn({
		let tx = tx.clone();
		move || read_physical_keyboard(tx, keyboard)
	});

	thread::spawn({
		let tx = tx.clone();
		move || read_physical_mousedev(tx, mousedev)
	});

	loop {
		let remaining = match runtime.deadline_remaining() {
			Some(duration) => duration,
			None => {
				runtime.reset_deadline();
				continue;
			},
		};

		let message = match rx.recv_timeout(remaining) {
			Ok(msg) => msg,
			Err(mpsc::RecvTimeoutError::Timeout) => {
				runtime.on_timeout();
				runtime.reset_deadline();
				continue;
			},
			Err(mpsc::RecvTimeoutError::Disconnected) => break,
		};

		if matches!(message.token, InputToken::MouseDelta(..)) {
			runtime.passthrough(&message);
			continue;
		}

		runtime.handle_input_values(&message);

		if runtime.is_quit_combo() {
			break;
		}

		runtime.handle_input_states(&message);

		runtime.clean_delta(&message);
	}
}
