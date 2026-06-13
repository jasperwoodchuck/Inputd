use crate::lang::{
	ast::Action,
	error::LangError,
	lexer::Token,
	parser::core::Parser,
};

impl Parser {
	pub(crate) fn parse_action(&mut self) -> Result<Action, LangError> {
		match self.current_token() {
			Some(Token::Disable) => {
				self.advance();
				Ok(Action::Disable)
			},

			Some(Token::Ident(_) | Token::Digit(_)) => Ok(Action::Emit(self.parse_hotkey()?)),

			Some(_) => Err(self.error("expected action".into())),
			None => Err(self.error("expected action, found EOF".into())),
		}
	}
}
