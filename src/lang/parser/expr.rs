use crate::{
	input::mapping::str_to_input_token,
	lang::{
		ast::{
			Binding,
			Program,
		},
		error::LangError,
		lexer::Token,
		parser::core::Parser,
	},
	types::input::Hotkey,
};

impl Parser {
	pub fn parse_program(&mut self) -> Result<Program, LangError> {
		let mut bindings = Vec::<Binding>::new();
		let mut keyboard: Option<String> = None;
		let mut mousedev: Option<String> = None;

		loop {
			self.skip_eol();

			if self.pos >= self.tokens.len() {
				break;
			}

			match self.current_token() {
				Some(Token::Keyboard) => {
					if keyboard.is_some() {
						return Err(self.error("duplicate keyboard declaration".into()));
					}

					self.advance();
					self.expect(Token::Equal)?;

					keyboard = Some(self.parse_string()?);
				},

				Some(Token::MouseDev) => {
					if mousedev.is_some() {
						return Err(self.error("duplicate mousedev declaration".into()));
					}

					self.advance();
					self.expect(Token::Equal)?;

					mousedev = Some(self.parse_string()?);
				},

				Some(_) => {
					let binding = self.parse_binding()?;
					bindings.push(binding);
				},

				None => break,
			}
		}

		let keyboard =
			keyboard.ok_or_else(|| self.error_at(None, "missing keyboard declaration".into()))?;
		let mousedev =
			mousedev.ok_or_else(|| self.error_at(None, "missing mousedev declaration".into()))?;

		Ok(Program {
			bindings,
			keyboard,
			mousedev,
		})
	}

	fn parse_binding(&mut self) -> Result<Binding, LangError> {
		let hotkey = self.parse_hotkey()?;

		self.expect(Token::Rebind)?;

		let action = self.parse_action()?;

		Ok(Binding {
			hotkey,
			action,
		})
	}

	pub(crate) fn parse_hotkey(&mut self) -> Result<Hotkey, LangError> {
		let mut hotkey = Hotkey::new();

		loop {
			let name = match self.current_token() {
				Some(Token::Ident(ident)) => ident.clone(),
				Some(Token::Digit(digit)) => digit.to_string(),
				_ => return Err(self.error("expected input token".into())),
			};

			let input = str_to_input_token(&name).map_err(|err| self.error(err))?;

			hotkey.push(input);

			self.advance();

			match self.current_token() {
				Some(Token::Plus) => self.advance(),
				_ => break,
			}
		}

		Ok(hotkey)
	}

	pub(crate) fn parse_string(&mut self) -> Result<String, LangError> {
		match self.current_token() {
			Some(Token::StringLiteral(string)) => {
				let string = string.clone();
				self.advance();
				Ok(string)
			},

			_ => Err(self.error("expected string".into())),
		}
	}
}
