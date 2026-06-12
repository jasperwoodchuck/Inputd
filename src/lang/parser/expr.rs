use crate::{
	input::mapping::str_to_input_token,
	lang::{
		ast::{
			Binding,
			Program,
		},
		error::LangError,
		lexer::Token,
		parser::core::{
			Parser,
			SpannedToken,
		},
	},
	types::input::Hotkey,
};

impl Parser {
	pub fn parse_program(&mut self) -> Result<Program, LangError> {
		let mut bindings = Vec::<Binding>::new();

		loop {
			self.skip_eol();

			if self.pos >= self.tokens.len() {
				break;
			}

			let binding = self.parse_binding()?;
			bindings.push(binding);
		}

		Ok(Program {
			bindings,
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
			let name = match self.current() {
				Some(SpannedToken {
					token: Token::Ident(ident),
					..
				}) => ident.clone(),
				Some(SpannedToken {
					token: Token::Digit(digit),
					..
				}) => digit.to_string(),
				_ => {
					return Err(self.error("expected input token".into()));
				},
			};

			let input = str_to_input_token(&name).map_err(|err| self.error(err))?;

			hotkey.push(input);

			self.advance();

			match self.current() {
				Some(SpannedToken {
					token: Token::Plus,
					..
				}) => self.advance(),
				_ => break,
			}
		}

		Ok(hotkey)
	}
}
