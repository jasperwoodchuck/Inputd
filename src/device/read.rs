use std::sync::mpsc::Sender;

use evdev::{
    Device,
    EventSummary,
    RelativeAxisCode,
};

use crate::utils::types::{
    Axis,
    AxisDirection,
    InputMessage,
    InputToken,
    InputValue,
};

pub fn read_keyboard(tx: Sender<InputMessage>, mut keyboard: Device) {
    loop {
        for event in keyboard.fetch_events().unwrap() {
            match event.destructure() {
                EventSummary::Key(_, key, value) => tx
                    .send(InputMessage {
                        token: InputToken::Key(key),
                        value: match value {
                            1 => InputValue::Press,
                            0 => InputValue::Release,
                            2 => InputValue::Repeat,
                            _ => InputValue::Release,
                        },
                    })
                    .unwrap(),
                _ => {}
            }
        }
    }
}

fn emit_scroll(tx: &Sender<InputMessage>, axis: Axis, direction: AxisDirection) {
    tx.send(InputMessage {
        token: InputToken::Scroll(axis, direction),
        value: match direction {
            AxisDirection::Positive => InputValue::Delta(1),
            AxisDirection::Negative => InputValue::Delta(-1),
        },
    })
    .unwrap();
}

fn handle_relative_axis(tx: &Sender<InputMessage>, axis_code: RelativeAxisCode, value: i32) {
    match axis_code {
        RelativeAxisCode::REL_WHEEL if value > 0 => {
            emit_scroll(tx, Axis::Y, AxisDirection::Positive);
        }

        RelativeAxisCode::REL_WHEEL if value < 0 => {
            emit_scroll(tx, Axis::Y, AxisDirection::Negative);
        }

        RelativeAxisCode::REL_HWHEEL if value > 0 => {
            emit_scroll(tx, Axis::X, AxisDirection::Positive);
        }

        RelativeAxisCode::REL_HWHEEL if value < 0 => {
            emit_scroll(tx, Axis::X, AxisDirection::Negative);
        }

        RelativeAxisCode::REL_X => {
            tx.send(InputMessage {
                token: InputToken::MouseMove(Axis::X),
                value: InputValue::Delta(value),
            })
            .unwrap();
        }

        RelativeAxisCode::REL_Y => {
            tx.send(InputMessage {
                token: InputToken::MouseMove(Axis::Y),
                value: InputValue::Delta(value),
            })
            .unwrap();
        }

        _ => {}
    }
}

pub fn read_dpimouse(tx: Sender<InputMessage>, mut dpimouse: Device) {
    loop {
        for event in dpimouse.fetch_events().unwrap() {
            match event.destructure() {
                EventSummary::Key(_, key, value) => tx
                    .send(InputMessage {
                        token: InputToken::Key(key),
                        value: match value {
                            1 => InputValue::Press,
                            0 => InputValue::Release,
                            2 => InputValue::Repeat,
                            _ => InputValue::Release,
                        },
                    })
                    .unwrap(),

                EventSummary::RelativeAxis(_, axis_code, value) => {
                    handle_relative_axis(&tx, axis_code, value)
                }
                _ => {}
            }
        }
    }
}
