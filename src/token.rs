use std::fmt::Display;

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
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
// pub fn empty_illegal() -> Token {
//     return Token::Illegal(String::from(""));
// }
pub fn empty_ident() -> Token {
    return Token::Ident(String::from(""));
}
// pub fn empty_int() -> Token {
//     return Token::Int(String::from(""));
// }
// pub fn empty_string() -> Token {
//     return Token::String(String::from(""));
// }

impl Token {
    pub fn literal(&self) -> &str {
        match self {
            Token::Illegal(val) => val,
            Token::Ident(val) => val,
            Token::Int(val) => val,
            Token::String(val) => val,
            Token::Eof => "",
            Token::Assign => "=",
            Token::Plus => "+",
            Token::Minus => "-",
            Token::Slash => "/",
            Token::Asterisk => "*",
            Token::Lt => "<",
            Token::Gt => ">",
            Token::Bang => "!",
            Token::Eq => "==",
            Token::NotEq => "!=",
            Token::Comma => ",",
            Token::Semicolon => ";",
            Token::Lparen => "(",
            Token::Rparen => ")",
            Token::Lbrace => "{",
            Token::Rbrace => "}",
            Token::Lbracket => "[",
            Token::Rbracket => "]",
            Token::Colon => ":",
            Token::Function => "fn",
            Token::Let => "let",
            Token::If => "if",
            Token::Else => "else",
            Token::Return => "return",
            Token::True => "true",
            Token::False => "false",
        }
    }
}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Token::Illegal(val) => write!(f, "Illegal({})", val),
            Token::Ident(val) => write!(f, "Ident({})", val),
            Token::Int(val) => write!(f, "Int({})", val),
            Token::String(val) => write!(f, "String({})", val),
            _ => write!(f, "{}", self.literal()),
        }
    }
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
