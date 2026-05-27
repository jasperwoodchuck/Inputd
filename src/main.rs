use logos::Logos;

use crate::lang::{
    lexer::Token,
    parser::Parser,
};

mod config;
mod lang;

fn main() {
    let content = config::load_config_file();

    let tokens = Token::lexer(&content)
        .map(|t| match t {
            Ok(token) => token,
            Err(err) => panic!("{:?}", err),
        })
        .collect::<Vec<Token>>();

    let mut parser = Parser::new(tokens);

    let bindings = parser.parse();
}
