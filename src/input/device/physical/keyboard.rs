use std::sync::mpsc;

use evdev::{
	Device,
	EventSummary,
};

use crate::types::input::{
	InputMessage,
	InputToken,
	InputValue,
};

pub fn read_physical_keyboard(tx: mpsc::Sender<InputMessage>, mut keyboard: Device) {
	loop {
		let events = match keyboard.fetch_events() {
			Ok(events) => events,
			Err(err) => {
				eprintln!("failed to fetch events: {err}");
				return;
			},
		};

		for event in events {
			if let EventSummary::Key(_, key, value) = event.destructure() {
				let value = match value {
					1 => InputValue::Press,
					0 => InputValue::Release,
					2 => InputValue::Repeat,
					_ => InputValue::Release,
				};

				if tx
					.send(InputMessage {
						token: InputToken::Key(key),
						value,
					})
					.is_err()
				{
					return;
				}
			}
		}
	}
}
