use evdev::KeyCode;

#[derive(Debug, Clone, Copy)]
pub enum Axis {
	X,
	Y,
}

#[derive(Debug, Clone, Copy)]
pub enum Magnitude {
	Positive,
	Negative,
}

#[derive(Debug, Clone, Copy)]
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
