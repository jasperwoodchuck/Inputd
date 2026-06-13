use crate::types::input::Hotkey;

#[derive(Debug)]
pub struct Program {
	pub bindings: Vec<Binding>,
	pub keyboard: String,
	pub mousedev: String,
}

#[derive(Debug)]
pub struct Binding {
	pub hotkey: Hotkey,
	pub action: Action,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Action {
	Emit(Hotkey),
	Disable,
}
