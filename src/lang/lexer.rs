use logos::Logos;

#[derive(Logos, Debug, PartialEq, Clone)]
#[logos(skip r"[ \t\f]+")]
pub enum Token {
    #[token("\n")]
    EOL,

    #[token("+")]
    Plus,

    #[token("::")]
    Rebind,

    #[regex(r"[A-Z0-9_]+", |lex| lex.slice().to_string())]
    Ident(String),
}
