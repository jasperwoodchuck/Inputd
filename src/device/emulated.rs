use evdev::{
    AttributeSet,
    KeyCode,
    RelativeAxisCode,
    uinput::VirtualDevice,
};

pub fn virtual_keyboard() -> VirtualDevice {
    let mut keys = AttributeSet::<KeyCode>::new();

    for code in 0..=0x2ff {
        keys.insert(KeyCode::new(code));
    }

    //TODO: Use proper error handling instead of .unwarp()
    VirtualDevice::builder()
        .unwrap()
        .name("inputd virtual keyboard")
        .with_keys(&keys)
        .unwrap()
        .build()
        .unwrap()
}

pub fn virtual_dpimouse() -> VirtualDevice {
    let mut buttons = AttributeSet::<KeyCode>::new();

    buttons.insert(KeyCode::BTN_LEFT);
    buttons.insert(KeyCode::BTN_MIDDLE);
    buttons.insert(KeyCode::BTN_RIGHT);

    buttons.insert(KeyCode::BTN_SIDE);
    buttons.insert(KeyCode::BTN_EXTRA);

    let mut axes = AttributeSet::<RelativeAxisCode>::new();

    axes.insert(RelativeAxisCode::REL_X);
    axes.insert(RelativeAxisCode::REL_Y);

    axes.insert(RelativeAxisCode::REL_WHEEL);
    axes.insert(RelativeAxisCode::REL_HWHEEL);

    //TODO: Use proper error handling instead of .unwarp()
    VirtualDevice::builder()
        .unwrap()
        .name("dpimouse")
        .with_keys(&buttons)
        .unwrap()
        .with_relative_axes(&axes)
        .unwrap()
        .build()
        .unwrap()
}
