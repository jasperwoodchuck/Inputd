use crate::{
	config::loader::load_config,
	input::{
		device::{
			emulated::{
				virtual_keyboard,
				virtual_mousedev,
			},
			physical::open::open_device,
		},
		rebind,
	},
	lang::{
		lexer::tokenize,
		parser::core::Parser,
	},
};

mod config;
mod input;
mod lang;
mod types;

fn main() {
	let passthrough_inputs = true;

	let keyboard_path = "";
	let mousedev_path = "";

	let mut keyboard = unwrap_or!(open_device(keyboard_path), return);
	let mut mousedev = unwrap_or!(open_device(mousedev_path), return);

	if !passthrough_inputs {
		keyboard.grab().expect("failed to grab keyboard");
		mousedev.grab().expect("failed to grab mouse");
	}

	let virtual_keyboard = unwrap_or!(virtual_keyboard(), return);
	let virtual_mousedev = unwrap_or!(virtual_mousedev(), return);

	let source = unwrap_or!(load_config(), return);
	let tokens = unwrap_or!(tokenize(&source), return);

	let mut parser = Parser::new(tokens, source);

	let config = unwrap_or!(parser.parse_program(), return);

	rebind::engine::start(keyboard, mousedev);
}
