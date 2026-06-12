use logos::Span;

use crate::lang::{
	error::LangError,
	lexer::Token,
};

pub struct SpannedToken {
	pub token: Token,
	pub span: Span,
}

pub struct Parser {
	pub tokens: Vec<SpannedToken>,
	pub pos: usize,
	pub source: String,
}

impl Parser {
	pub fn new(tokens: Vec<SpannedToken>, source: String) -> Self {
		Self {
			tokens,
			pos: 0,
			source,
		}
	}

	pub(crate) fn current(&self) -> Option<&SpannedToken> {
		self.tokens.get(self.pos)
	}

	pub(crate) fn advance(&mut self) {
		self.pos += 1;
	}

	pub(crate) fn skip_eol(&mut self) {
		while let Some(spanned_token) = self.current() {
			if spanned_token.token != Token::Eol {
				break;
			}

			self.advance();
		}
	}

	pub(crate) fn error(&self, message: String) -> LangError {
		let span = self.current().map(|token| token.span.clone());

		LangError {
			message,
			span,
			source: self.source.clone(),
		}
	}

	pub(crate) fn expect(&mut self, expected: Token) -> Result<(), LangError> {
		match self.current() {
			Some(spanned_token) if spanned_token.token == expected => {
				self.advance();
				Ok(())
			},
			Some(spanned_token) => Err(self.error(format!(
				"expected {expected:?}, found {:?}",
				spanned_token.token
			))),
			None => Err(self.error(format!("expected {expected:?}, found EOF"))),
		}
	}
}
