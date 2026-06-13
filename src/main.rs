use crate::{
	config::loader::load_config,
	input::{
		device::{
			emulated::{
				virtual_keyboard,
				virtual_mousedev,
			},
			physical::open::{
				open_device,
				wait_for_clean_input_state,
			},
		},
		rebind,
	},
	lang::{
		lexer::tokenize,
		parser::core::Parser,
	},
	utils::helper::binding_to_dict,
};

mod config;
mod input;
mod lang;
mod types;
mod utils;

fn main() {
	let source = unwrap_or!(load_config(), return);
	let tokens = unwrap_or!(tokenize(&source), return);

	let mut parser = Parser::new(tokens, source);

	let config = unwrap_or!(parser.parse_program(), return);
	let passthrough_inputs = false;

	let keyboard_path = &config.keyboard;
	let mousedev_path = &config.mousedev;

	let mut keyboard = unwrap_or!(open_device(keyboard_path), return);
	let mut mousedev = unwrap_or!(open_device(mousedev_path), return);

	wait_for_clean_input_state(&keyboard, &mousedev);

	if !passthrough_inputs {
		unwrap_or!(keyboard.grab(), return);
		unwrap_or!(mousedev.grab(), return);
	}

	let virtual_keyboard = unwrap_or!(virtual_keyboard(), return);
	let virtual_mousedev = unwrap_or!(virtual_mousedev(), return);

	let rebind_dict = binding_to_dict(config.bindings);

	rebind::engine::start(
		rebind_dict,
		virtual_keyboard,
		virtual_mousedev,
		keyboard,
		mousedev,
	);
}
