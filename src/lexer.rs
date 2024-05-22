use std::str::CharIndices;

use crate::token::{self, Token};

pub struct Lexer<'a> {
    input: &'a str,
    iter: CharIndices<'a>,
    length: usize,
    position: usize,
    ch: char,
}

impl Lexer<'_> {
    pub fn new<'a>(input: &'a str) -> Lexer<'a> {
        let length = input.len();
        let mut l = Lexer {
            iter: input.char_indices(),
            input,
            length: length,
            ch: '\0',
            position: 0,
        };
        l.read_char();
        return l;
    }

    fn read_char(&mut self) {
        let (pos, ch) = self.iter.next().unwrap_or((self.length, '\0'));
        self.position = pos;
        self.ch = ch;
    }

    fn peek_char(&mut self) -> char {
        match self.iter.clone().next() {
            None => '\0',
            Some((_, ch)) => ch,
        }
    }

    fn skip_whitespace(&mut self) {
        while self.ch.is_whitespace() {
            self.read_char()
        }
    }

    fn decide_2_char_token(&mut self, next_char: char, token: Token) -> Option<Token> {
        if self.peek_char() == next_char {
            self.read_char();
            return Some(token);
        } else {
            return None;
        }
    }

    pub fn next_token(&mut self) -> Token {
        let tok: Token;
        self.skip_whitespace();

        match self.ch {
            '=' => {
                tok = self
                    .decide_2_char_token('=', Token::Eq)
                    .unwrap_or(Token::Assign)
            }
            '!' => {
                tok = self
                    .decide_2_char_token('=', Token::NotEq)
                    .unwrap_or(Token::Bang)
            }
            ';' => tok = Token::Semicolon,
            '(' => tok = Token::Lparen,
            ')' => tok = Token::Rparen,
            ',' => tok = Token::Comma,
            '{' => tok = Token::Lbrace,
            '}' => tok = Token::Rbrace,
            '+' => tok = Token::Plus,
            '-' => tok = Token::Minus,
            '/' => tok = Token::Slash,
            '*' => tok = Token::Asterisk,
            '<' => tok = Token::Lt,
            '>' => tok = Token::Gt,
            '[' => tok = Token::Lbracket,
            ']' => tok = Token::Rbracket,
            ':' => tok = Token::Colon,
            '"' => tok = Token::String(self.read_string()),
            '\0' => tok = Token::Eof,
            ch => {
                if ch.is_letter() {
                    tok = token::lookup_ident(self.read_identifier())
                } else if ch.is_ascii_digit() {
                    tok = Token::Int(self.read_number())
                } else {
                    tok = Token::Illegal(ch.to_string())
                }
            }
        }
        self.read_char();
        return tok;
    }

    fn read_string(&mut self) -> String {
        let start = self.position;
        loop {
            self.read_char();
            if self.ch == '"' || self.ch == '\0' {
                break;
            }
        }
        return self.input[start..self.position].to_string();
    }

    fn read_identifier(&mut self) -> &str {
        let start = self.position;
        while self.ch.is_letter() {
            self.read_char();
        }
        return &self.input[start..self.position];
    }

    fn read_number(&mut self) -> String {
        let start = self.position;
        while self.ch.is_ascii_digit() {
            self.read_char();
        }
        return self.input[start..self.position].to_string();
    }
}

impl Iterator for Lexer<'_> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        match self.next_token() {
            Token::Eof => None,
            tok => Some(tok),
        }
    }
}

trait LexerChar {
    fn is_letter(&self) -> bool;
}

impl LexerChar for char {
    fn is_letter(&self) -> bool {
        self.is_alphabetic() || *self == '_'
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn next_token_single_char() {
        let input = "=+(){},;";
        let expected = [
            Token::Eq,
            Token::Plus,
            Token::Lparen,
            Token::Rparen,
            Token::Lbrace,
            Token::Rbrace,
            Token::Comma,
            Token::Semicolon,
        ];

        let mut l = Lexer::new(input);
        for exp in expected {
            let tok = l.next_token();
            assert_eq!(exp, tok);
        }
    }
}
