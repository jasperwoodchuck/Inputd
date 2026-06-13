use std::str;

use logos::{
	Lexer,
	Logos,
	Skip,
};

use crate::lang::{
	error::LangError,
	parser::core::SpannedToken,
};

fn parse<T>(lex: &mut Lexer<Token>) -> Option<T>
where T: str::FromStr {
	lex.slice().parse().ok()
}

fn skip_comment(lex: &mut Lexer<Token>) -> Skip {
	let rest = lex.remainder();

	let len = rest.find("\n").unwrap_or(rest.len());
	lex.bump(len);

	Skip
}

fn unquote_string(lex: &mut Lexer<Token>) -> String {
	let slice = lex.slice();
	slice[1..slice.len() - 1].to_owned()
}

#[derive(Logos, Debug, PartialEq, Clone)]
pub enum Token {
	#[regex("#", skip_comment)]
	Comment,

	#[token("+")]
	Plus,

	#[token("=")]
	Equal,

	#[token(",")]
	Comma,

	#[token("(")]
	LParen,

	#[token(")")]
	RParen,

	#[regex(r"\n+")]
	Eol,

	#[token("::")]
	Rebind,

	#[token("Keyboard")]
	Keyboard,

	#[token("Mouse")]
	MouseDev,

	#[token("Disable")]
	Disable,

	#[regex(r"[ \t\f]+", logos::skip)]
	Whitespace,

	#[regex("[A-Za-z][A-Za-z0-9_]*", parse)]
	Ident(String),

	#[regex("[0-9]+", parse)]
	Digit(u32),

	#[regex(r#""[^"]*""#, unquote_string)]
	StringLiteral(String),
}

pub fn tokenize(src: &String) -> Result<Vec<SpannedToken>, LangError> {
	let mut lexer = Token::lexer(src);
	let mut spanned_tokens = Vec::<SpannedToken>::new();

	while let Some(spanned_token) = lexer.next() {
		let span = lexer.span();

		match spanned_token {
			Ok(token) => {
				spanned_tokens.push(SpannedToken {
					token,
					span,
				});
			},

			Err(_) => {
				return Err(LangError {
					message: "invalid token".to_string(),
					span: Some(span),
					source: src.to_owned(),
				});
			},
		}
	}

	Ok(spanned_tokens)
}
