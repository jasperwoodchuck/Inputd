use crate::{
	config::loader::load_config,
	lang::{
		lexer::tokenize,
		parser::core::Parser,
	},
};

mod config;
mod input;
mod lang;
mod types;

fn main() {
	let source = unwrap_or!(load_config(), return);
	let tokens = unwrap_or!(tokenize(&source), return);

	let mut parser = Parser::new(tokens, source);

	let config = unwrap_or!(parser.parse_program(), return);

	println!("{config:#?}")
}
