use crate::token::Token;

#[derive(PartialEq, PartialOrd)]
pub enum Precedence {
    Lowest = 1,
    Equals = 2,
    LessGreater = 3,
    Sum = 4,
    Product = 5,
    Prefix = 6,
    Call = 7,
    Index = 8,
}

fn precedences(tok: &Token) -> Precedence {
    match tok {
        Token::Eq => Precedence::Equals,
        Token::NotEq => Precedence::Equals,
        Token::Lt => Precedence::LessGreater,
        Token::Gt => Precedence::LessGreater,
        Token::Plus => Precedence::Sum,
        Token::Minus => Precedence::Sum,
        Token::Slash => Precedence::Product,
        Token::Asterisk => Precedence::Product,
        Token::Lparen => Precedence::Call,
        Token::Lbracket => Precedence::Index,
        _ => Precedence::Lowest,
    }
}
