use std::{
    sync::mpsc,
    thread,
};

use evdev::{
    Device,
    uinput::VirtualDevice,
};

use crate::{
    device::read::{
        read_dpimouse,
        read_keyboard,
    },
    utils::{
        mappings::token_to_event,
        types::{
            Axis,
            InputMessage,
            InputToken,
            InputValue,
            KeysVector,
            RebindDict,
        },
    },
};

fn is_strict_match(pressed: &KeysVector, combo: &KeysVector) -> bool {
    pressed.len() == combo.len() && pressed.iter().zip(combo).all(|(a, b)| a == b)
}

fn emit_remap(
    virtual_keyboard: &mut VirtualDevice,
    virtual_dpimouse: &mut VirtualDevice,
    input_token: InputToken,
    input_value: InputValue,
) {
    let input_value_i32 = match input_value {
        InputValue::Press => 1,
        InputValue::Release => 0,
        InputValue::Repeat => 2,
        InputValue::Delta(value) => value,
    };

    let input_event = token_to_event(input_token, input_value_i32);

    let device = match input_token {
        InputToken::Key(_) => virtual_keyboard,
        _ => virtual_dpimouse,
    };

    device.emit(&[input_event]).unwrap();
}

fn is_modifier(token: InputToken) -> bool {
    matches!(
        token,
        InputToken::Key(evdev::KeyCode::KEY_LEFTMETA)
            | InputToken::Key(evdev::KeyCode::KEY_LEFTALT)
            | InputToken::Key(evdev::KeyCode::KEY_RIGHTALT)
            | InputToken::Key(evdev::KeyCode::KEY_LEFTSHIFT)
            | InputToken::Key(evdev::KeyCode::KEY_RIGHTSHIFT)
            | InputToken::Key(evdev::KeyCode::KEY_LEFTCTRL)
            | InputToken::Key(evdev::KeyCode::KEY_RIGHTCTRL)
    )
}

fn emit_remap_sequence(
    virtual_keyboard: &mut VirtualDevice,
    virtual_dpimouse: &mut VirtualDevice,
    keycombo: &KeysVector,
    pressed: &KeysVector,
) {
    let held_modifiers: Vec<InputToken> = pressed
        .iter()
        .copied()
        .filter(|k| is_modifier(*k))
        .collect();

    for key in &held_modifiers {
        emit_remap(
            virtual_keyboard,
            virtual_dpimouse,
            *key,
            InputValue::Release,
        );
    }

    for key in keycombo {
        emit_remap(virtual_keyboard, virtual_dpimouse, *key, InputValue::Press);
    }

    for key in keycombo.iter().rev() {
        emit_remap(
            virtual_keyboard,
            virtual_dpimouse,
            *key,
            InputValue::Release,
        );
    }

    for key in &held_modifiers {
        emit_remap(virtual_keyboard, virtual_dpimouse, *key, InputValue::Press);
    }
}

fn insert_token(pressed: &mut KeysVector, input_msg: &InputMessage) {
    if !pressed.contains(&input_msg.token) {
        pressed.push(input_msg.token);
    }
}

fn remove_token(pressed: &mut KeysVector, input_msg: &InputMessage) {
    pressed.retain(|k| *k != input_msg.token);
}

pub fn start(
    mut virtual_keyboard: VirtualDevice,
    mut virtual_dpimouse: VirtualDevice,
    keyboard: Device,
    dpimouse: Device,
    config_table: RebindDict,
) {
    let (tx, rx) = mpsc::channel::<InputMessage>();

    let mut pressed = KeysVector::new();

    thread::spawn({
        let tx = tx.clone();
        move || read_keyboard(tx, keyboard)
    });

    thread::spawn({
        let tx = tx.clone();
        move || read_dpimouse(tx, dpimouse)
    });

    loop {
        let input_msg = rx.recv().unwrap();

        let mousemove = matches!(
            input_msg.token,
            InputToken::MouseMove(Axis::X) | InputToken::MouseMove(Axis::Y)
        );

        if mousemove {
            emit_remap(
                &mut virtual_keyboard,
                &mut virtual_dpimouse,
                input_msg.token,
                input_msg.value,
            );
        } else {
            match input_msg.value {
                InputValue::Press => {
                    insert_token(&mut pressed, &input_msg);
                }

                InputValue::Release => {
                    remove_token(&mut pressed, &input_msg);
                }

                InputValue::Repeat => {}
                InputValue::Delta(_) => {
                    insert_token(&mut pressed, &input_msg);
                }
            }

            let mut strict_match = false;

            for (original, remapped) in &config_table {
                if is_strict_match(&pressed, original) {
                    strict_match = true;
                    emit_remap_sequence(
                        &mut virtual_keyboard,
                        &mut virtual_dpimouse,
                        remapped,
                        &pressed,
                    );
                    break;
                }
            }

            if matches!(input_msg.value, InputValue::Delta(_)) {
                remove_token(&mut pressed, &input_msg);
            }

            if !strict_match {
                emit_remap(
                    &mut virtual_keyboard,
                    &mut virtual_dpimouse,
                    input_msg.token,
                    input_msg.value,
                );
            }
        }
    }
}
