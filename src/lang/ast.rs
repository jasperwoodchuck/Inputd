use crate::types::input::Hotkey;

#[derive(Debug)]
pub struct Program {
	pub bindings: Vec<Binding>,
}

#[derive(Debug)]
pub struct Binding {
	pub hotkey: Hotkey,
	pub action: Action,
}

#[derive(Debug)]
pub enum Action {
	Emit(Hotkey),
	Disable,
}
