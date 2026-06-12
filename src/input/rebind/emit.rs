use evdev::uinput::VirtualDevice;

use crate::{
	input::mapping::{
		input_token_to_event,
		input_value_to_i32,
	},
	types::input::{
		InputToken,
		InputValue,
	},
};

pub fn emit_input(
	virtual_keyboard: &mut VirtualDevice,
	virtual_mousedev: &mut VirtualDevice,
	input_token: &InputToken,
	input_value: &InputValue,
) -> std::io::Result<()> {
	let value = input_value_to_i32(input_value);
	let event = input_token_to_event(input_token, value);

	match input_token {
		InputToken::Key(_) => virtual_keyboard,
		InputToken::Btn(_) => virtual_mousedev,
		InputToken::MouseDelta(..) => virtual_mousedev,
		InputToken::MouseWheel(..) => virtual_mousedev,
	}
	.emit(&[event])
}

pub fn emit_input_sequence(
	virtual_keyboard: &mut VirtualDevice,
	virtual_mousedev: &mut VirtualDevice,
	input_tokens: &[InputToken],
) -> std::io::Result<()> {
	for input_token in input_tokens {
		emit_input(
			virtual_keyboard,
			virtual_mousedev,
			input_token,
			&InputValue::Press,
		)?;
	}

	for input_token in input_tokens.iter().rev() {
		emit_input(
			virtual_keyboard,
			virtual_mousedev,
			input_token,
			&InputValue::Release,
		)?;
	}

	Ok(())
}
