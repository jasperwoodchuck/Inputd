use std::time::{
	Duration,
	Instant,
};

use evdev::{
	KeyCode,
	uinput::VirtualDevice,
};

use crate::{
	input::rebind::emit::{
		emit_input,
		emit_input_sequence,
	},
	lang::ast::Action,
	types::input::{
		InputMessage,
		InputToken,
		InputValue,
		MatchMode,
		RebindDict,
	},
	utils::helper::{
		insert_token,
		remove_token,
	},
};

pub struct Runtime {
	rebind_dict: RebindDict,

	timeout_duration: Duration,
	deadline: Instant,

	virtual_keyboard: VirtualDevice,
	virtual_mousedev: VirtualDevice,

	pressed: Vec<InputToken>,
	pending: Option<Action>,
}

impl Runtime {
	const TIMEOUT_MS: u64 = 150;

	pub(crate) fn new(
		rebind_dict: RebindDict,

		virtual_keyboard: VirtualDevice,
		virtual_mousedev: VirtualDevice,
	) -> Self {
		let timeout_duration = Duration::from_millis(Self::TIMEOUT_MS);

		Self {
			rebind_dict,
			timeout_duration,
			deadline: Instant::now() + timeout_duration,
			virtual_keyboard,
			virtual_mousedev,
			pressed: Vec::<InputToken>::new(),
			pending: None,
		}
	}

	pub(crate) fn deadline_remaining(&self) -> Option<Duration> {
		let now = Instant::now();
		self.deadline.checked_duration_since(now)
	}

	pub(crate) fn reset_deadline(&mut self) {
		self.deadline = Instant::now() + self.timeout_duration;
	}

	pub(crate) fn passthrough(&mut self, message: &InputMessage) {
		emit_input(
			&mut self.virtual_keyboard,
			&mut self.virtual_mousedev,
			&message.token,
			&message.value,
		);
	}

	fn compare_pressed(&self) -> MatchMode {
		if self.pressed.is_empty() {
			return MatchMode::None;
		}

		let mut strict = false;
		let mut prefix = false;

		for combo in self.rebind_dict.keys() {
			if self.pressed == *combo {
				strict = true;
			} else if combo.starts_with(&self.pressed) {
				prefix = true;
			}
		}

		match (strict, prefix) {
			(true, true) => MatchMode::Both,
			(true, false) => MatchMode::Strict,
			(false, true) => MatchMode::Prefix,
			(false, false) => MatchMode::None,
		}
	}

	fn execute_action(&mut self, action: Action) {
		match action {
			Action::Emit(remapped) => {
				emit_input_sequence(
					&mut self.virtual_keyboard,
					&mut self.virtual_mousedev,
					&self.pressed,
					&remapped,
				);
			},

			Action::Disable => {},
		}
	}

	pub(crate) fn handle_input_values(&mut self, message: &InputMessage) {
		match message.value {
			InputValue::Press => {
				insert_token(&mut self.pressed, message);
				self.reset_deadline();
			},

			InputValue::Release => {
				remove_token(&mut self.pressed, message);
				self.reset_deadline();
			},

			InputValue::Repeat => {},
			InputValue::Delta(_) => {
				insert_token(&mut self.pressed, message);
			},
		}
	}

	pub(crate) fn handle_input_states(&mut self, message: &InputMessage) {
		match self.compare_pressed() {
			MatchMode::None => {
				self.passthrough(message);
				self.pending = None;
			},

			MatchMode::Both => {
				if matches!(message.value, InputValue::Release) {
					self.passthrough(message);
				}
				self.pending = self.rebind_dict.get(&self.pressed).cloned();
			},

			MatchMode::Prefix => {
				if matches!(message.value, InputValue::Release) {
					self.passthrough(message);
				}
				self.pending = Some(Action::Emit(self.pressed.clone()));
			},

			MatchMode::Strict => {
				if let Some(action) = self.rebind_dict.get(&self.pressed).cloned() {
					self.execute_action(action);
				}
				self.pending = None;
			},
		}
	}

	pub fn is_quit_combo(&self) -> bool {
		const QUIT_COMBO: &[InputToken] = &[
			InputToken::Key(KeyCode::KEY_LEFTCTRL),
			InputToken::Key(KeyCode::KEY_LEFTALT),
			InputToken::Key(KeyCode::KEY_LEFTSHIFT),
			InputToken::Key(KeyCode::KEY_DELETE),
		];
		self.pressed.len() == QUIT_COMBO.len()
			&& QUIT_COMBO.iter().all(|input| self.pressed.contains(input))
	}

	pub(crate) fn on_timeout(&mut self) {
		if let Some(action) = self.pending.take() {
			self.execute_action(action);
		}
		self.pending = None;
	}

	pub fn clean_delta(&mut self, input_msg: &InputMessage) {
		if matches!(input_msg.value, InputValue::Delta(_)) {
			remove_token(&mut self.pressed, input_msg);
		}
	}
}
