use std::collections::HashMap;

use evdev::KeyCode;

#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
pub enum Axis {
    X,
    Y,
}

#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
pub enum AxisDirection {
    Positive,
    Negative,
}

#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
pub enum InputToken {
    Key(KeyCode),
    MouseMove(Axis),
    Scroll(Axis, AxisDirection),
}

#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
pub enum InputValue {
    Press,
    Release,
    Repeat,
    Delta(i32),
}

pub struct InputMessage {
    pub token: InputToken,
    pub value: InputValue,
}

pub type KeysVector = Vec<InputToken>;
pub type RebindDict = HashMap<KeysVector, KeysVector>;
