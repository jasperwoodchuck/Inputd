use crate::lang::{
    ast::Binding,
    lexer::Token,
};

pub struct Parser {
    tokens: Vec<Token>,
    pos: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, pos: 0 }
    }

    pub fn peek(&self) -> Option<&Token> {
        self.tokens.get(self.pos)
    }

    fn advance(&mut self) -> Option<Token> {
        let token = self.tokens.get(self.pos).cloned();
        self.pos += 1;
        token
    }

    fn parse_combo(&mut self) -> Vec<String> {
        let mut keys = Vec::new();

        loop {
            match self.advance() {
                Some(Token::Ident(key)) => {
                    keys.push(key);
                }

                _ => panic!("expected key"),
            }

            match self.peek() {
                Some(Token::Plus) => {
                    self.advance();
                }

                _ => break,
            }
        }

        keys
    }

    pub fn parse_binding(&mut self) -> Binding {
        let original = self.parse_combo();

        match self.advance() {
            Some(Token::Rebind) => {}

            _ => panic!("expected ::"),
        }

        let remapped = self.parse_combo();

        Binding { original, remapped }
    }

    pub fn parse(&mut self) -> Vec<Binding> {
        let mut bindings = Vec::new();

        while self.pos < self.tokens.len() {
            bindings.push(self.parse_binding());

            match self.peek() {
                Some(Token::EOL) => {
                    self.advance();
                }

                None => break,

                _ => panic!("expected end of line"),
            }
        }

        bindings
    }
}
