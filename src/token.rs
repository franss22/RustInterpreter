#[derive(Debug, PartialEq)]
pub enum Token {
    Illegal(String),
    Eof,

    Ident(String),
    Int(String),
    String(String),

    Assign,
    Plus,
    Minus,
    Slash,
    Asterisk,
    Lt,
    Gt,

    Bang,

    Eq,
    NotEq,

    Comma,
    Semicolon,
    Lparen,
    Rparen,
    Lbrace,
    Rbrace,
    Lbracket,
    Rbracket,
    Colon,

    Function,
    Let,
    If,
    Else,
    Return,
    True,
    False,
}

pub fn lookup_ident(word: &str) -> Token {
    match word {
        "fn" => Token::Function,
        "let" => Token::Let,
        "if" => Token::If,
        "else" => Token::Else,
        "return" => Token::Return,
        "true" => Token::True,
        "false" => Token::False,
        _ => Token::Ident(word.to_string()),
    }
}
