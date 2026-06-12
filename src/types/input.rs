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
pub enum Input {
	Key(KeyCode),
	Btn(KeyCode),
	Delta(Axis, Magnitude),
	Wheel(Axis, Magnitude),
}

pub type Hotkey = Vec<Input>;
