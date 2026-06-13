use std::{
	fs::File,
	io,
	os::fd::OwnedFd,
	thread,
	time::Duration,
};

use evdev::Device;

use crate::unwrap_or;

pub fn open_device(path: &str) -> Result<Device, io::Error> {
	let device = File::open(format!("/dev/input/{}", path))?;
	let ofd = OwnedFd::from(device);

	Device::from_fd(ofd)
}

pub fn wait_for_clean_input_state(keyboard: &Device, mousedev: &Device) {
	loop {
		let keys = unwrap_or!(keyboard.get_key_state(), return);
		let btns = unwrap_or!(mousedev.get_key_state(), return);

		if keys.iter().next().is_none() && btns.iter().next().is_none() {
			break;
		}

		thread::sleep(Duration::from_millis(10));
	}
}
