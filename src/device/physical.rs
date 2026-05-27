use evdev::Device;
use std::{
    fs::File,
    os::fd::OwnedFd,
};

pub fn open_device(id: i8, grab: bool) -> Device {
    let f = File::open(format!("/dev/input/event{}", id)).unwrap();
    let fd = OwnedFd::from(f);

    let mut device = Device::from_fd(fd).unwrap();

    if grab {
        device.grab().unwrap();
    }

    device
}
