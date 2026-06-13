use evdev::uinput::VirtualDevice;

use crate::{
	input::mapping::input_to_event,
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
) {
	let event = input_to_event(input_token, input_value);

	let device = match input_token {
		InputToken::Key(_) => virtual_keyboard,
		InputToken::Btn(_) => virtual_mousedev,
		InputToken::MouseDelta(..) => virtual_mousedev,
		InputToken::MouseWheel(..) => virtual_mousedev,
	};

	if let Err(err) = device.emit(&[event]) {
		eprintln!("failed to emit {:?}: {}", input_token, err);
	}
}

fn emit_many(
	virtual_keyboard: &mut VirtualDevice,
	virtual_mousedev: &mut VirtualDevice,
	input_tokens: &[InputToken],
	input_value: InputValue,
) {
	for input_token in input_tokens {
		emit_input(
			virtual_keyboard,
			virtual_mousedev,
			input_token,
			&input_value,
		);
	}
}

fn emit_many_rev(
	virtual_keyboard: &mut VirtualDevice,
	virtual_mousedev: &mut VirtualDevice,
	input_tokens: &[InputToken],
	input_value: InputValue,
) {
	for input_token in input_tokens.iter().rev() {
		emit_input(
			virtual_keyboard,
			virtual_mousedev,
			input_token,
			&input_value,
		);
	}
}

pub fn emit_input_sequence(
	virtual_keyboard: &mut VirtualDevice,
	virtual_mousedev: &mut VirtualDevice,
	original_inputs: &[InputToken],
	remapped_inputs: &[InputToken],
) {
	let modifiers: Vec<_> = original_inputs
		.iter()
		.filter(|i| i.is_modifier())
		.copied()
		.collect();

	emit_many(
		virtual_keyboard,
		virtual_mousedev,
		&modifiers,
		InputValue::Release,
	);

	emit_many(
		virtual_keyboard,
		virtual_mousedev,
		remapped_inputs,
		InputValue::Press,
	);

	emit_many_rev(
		virtual_keyboard,
		virtual_mousedev,
		remapped_inputs,
		InputValue::Release,
	);

	emit_many(
		virtual_keyboard,
		virtual_mousedev,
		&modifiers,
		InputValue::Press,
	);
}
