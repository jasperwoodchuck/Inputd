use std::{
	fs::File,
	io,
	os::fd::OwnedFd,
};

use evdev::Device;

pub fn open_device(path: &str) -> Result<Device, io::Error> {
	let device = File::open(format!("/dev/input/{}", path))?;
	let ofd = OwnedFd::from(device);

	Device::from_fd(ofd)
}
