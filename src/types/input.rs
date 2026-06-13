use std::collections::HashMap;

use evdev::KeyCode;

use crate::lang::ast::Action;

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum Axis {
	X,
	Y,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum Magnitude {
	Positive,
	Negative,
}

#[derive(Debug)]
pub enum MatchMode {
	None,
	Both,
	Prefix,
	Strict,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum InputToken {
	Key(KeyCode),
	Btn(KeyCode),
	MouseDelta(Axis, Magnitude),
	MouseWheel(Axis, Magnitude),
}

#[derive(Debug)]
pub enum InputValue {
	Press,
	Release,
	Repeat,
	Delta(i32),
}

#[derive(Debug)]
pub struct InputMessage {
	pub token: InputToken,
	pub value: InputValue,
}

pub type Hotkey = Vec<InputToken>;
pub type RebindDict = HashMap<Hotkey, Action>;
