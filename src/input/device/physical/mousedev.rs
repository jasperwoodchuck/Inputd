use std::sync::mpsc;

use evdev::{
	Device,
	EventSummary,
	RelativeAxisCode,
};

use crate::{
	input::mapping::magnitude_to_i32,
	types::input::{
		Axis,
		InputMessage,
		InputToken,
		InputValue,
		Magnitude,
	},
};

fn handle_wheel(tx: &mpsc::Sender<InputMessage>, axis: Axis, magnitude: Magnitude) {
	let delta = magnitude_to_i32(&magnitude);

	tx.send(InputMessage {
		token: InputToken::MouseWheel(axis, magnitude),
		value: InputValue::Delta(delta),
	})
	.expect("failed to send input event");
}

fn handle_delta(tx: &mpsc::Sender<InputMessage>, axis: Axis, magnitude: Magnitude) {
	let delta = magnitude_to_i32(&magnitude);

	tx.send(InputMessage {
		token: InputToken::MouseDelta(axis, magnitude),
		value: InputValue::Delta(delta),
	})
	.expect("failed to send input event");
}

fn handle_relative_axis(tx: &mpsc::Sender<InputMessage>, axis_code: RelativeAxisCode, value: i32) {
	match axis_code {
		RelativeAxisCode::REL_WHEEL if value > 0 => {
			handle_wheel(tx, Axis::Y, Magnitude::Positive);
		},

		RelativeAxisCode::REL_WHEEL if value < 0 => {
			handle_wheel(tx, Axis::Y, Magnitude::Negative);
		},

		RelativeAxisCode::REL_HWHEEL if value > 0 => {
			handle_wheel(tx, Axis::X, Magnitude::Positive);
		},

		RelativeAxisCode::REL_HWHEEL if value < 0 => {
			handle_wheel(tx, Axis::X, Magnitude::Negative);
		},

		RelativeAxisCode::REL_X if value > 0 => {
			handle_delta(tx, Axis::X, Magnitude::Positive);
		},

		RelativeAxisCode::REL_X if value < 0 => {
			handle_delta(tx, Axis::X, Magnitude::Negative);
		},

		RelativeAxisCode::REL_Y if value > 0 => {
			handle_delta(tx, Axis::Y, Magnitude::Positive);
		},

		RelativeAxisCode::REL_Y if value < 0 => {
			handle_delta(tx, Axis::Y, Magnitude::Negative);
		},

		_ => {},
	}
}

pub fn read_physical_mousedev(tx: mpsc::Sender<InputMessage>, mut mousedev: Device) {
	loop {
		let events = match mousedev.fetch_events() {
			Ok(events) => events,
			Err(err) => {
				eprintln!("failed to fetch mouse events: {err}");
				return;
			},
		};

		for event in events {
			match event.destructure() {
				EventSummary::Key(_, key, value) => {
					let value = match value {
						1 => InputValue::Press,
						0 => InputValue::Release,
						2 => InputValue::Repeat,
						_ => InputValue::Release,
					};

					if tx
						.send(InputMessage {
							token: InputToken::Btn(key),
							value,
						})
						.is_err()
					{
						return;
					}
				},

				EventSummary::RelativeAxis(_, axis_code, value) => {
					handle_relative_axis(&tx, axis_code, value);
				},

				_ => {},
			}
		}
	}
}
