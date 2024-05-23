use crate::{
    ast::{self, Expression, Ident, Statement},
    lexer::Lexer,
    token::{self, Token},
};
use std::mem::discriminant;

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
type InfixParseFunc = fn(&mut Parser, ast::Expression) -> Option<ast::Expression>;
type PrefixParseFunc = fn(&mut Parser) -> Option<ast::Expression>;

pub struct Parser<'a> {
    lexer: Lexer<'a>,
    curr_token: Token,
    peek_token: Token,
    errors: Vec<String>,
}

impl Parser<'_> {
    pub fn new<'a>(mut lexer: Lexer<'a>) -> Parser<'a> {
        let curr_token = lexer.next_token();
        let peek_token = lexer.next_token();
        let errors: Vec<String> = Vec::new();
        return Parser {
            lexer,
            curr_token,
            peek_token,
            errors,
        };
    }

    fn prefix_parse_fn(&mut self, tok: &token::Token) -> Option<PrefixParseFunc> {
        match tok {
            Token::Ident(_) => Some(Self::parse_identifier),
            Token::Int(_) => Some(Self::parse_integer_literal),
            Token::String(_) => Some(Self::parse_string_literal),
            Token::Minus => Some(Self::parse_prefix_expression),
            Token::Bang => Some(Self::parse_prefix_expression),
            Token::Lparen => todo!(),
            Token::Lbrace => todo!(),
            Token::Lbracket => todo!(),
            Token::Function => todo!(),
            Token::If => todo!(),
            Token::True => Some(Self::parse_boolean_literal),
            Token::False => Some(Self::parse_boolean_literal),
            _ => None,
        }
    }
    fn parse_prefix_expression(p: &mut Parser) -> Option<ast::Expression> {
        let operator = p.curr_token.clone();
        p.next_token();
        let right = Box::new(p.parse_expression(Precedence::Prefix)?);
        return Some(Expression::PrefixOperation { operator, right });
    }
    fn parse_identifier(p: &mut Parser) -> Option<Expression> {
        if let Token::Ident(name) = p.curr_token.clone() {
            Some(Expression::Identifier(Ident { name }))
        } else {
            unreachable!()
        }
    }

    fn parse_boolean_literal(p: &mut Parser) -> Option<Expression> {
        match p.curr_token {
            Token::True => Some(Expression::BooleanLiteral(true)),
            Token::False => Some(Expression::BooleanLiteral(false)),
            _ => unreachable!(),
        }
    }

    fn parse_integer_literal(p: &mut Parser) -> Option<Expression> {
        if let Token::Int(val) = p.curr_token.clone() {
            let int_val = val.parse::<i64>().unwrap();
            Some(Expression::IntegerLiteral(int_val))
        } else {
            unreachable!()
        }
    }

    fn parse_string_literal(p: &mut Parser) -> Option<Expression> {
        if let Token::String(val) = p.curr_token.clone() {
            Some(Expression::StringLiteral(val))
        } else {
            unreachable!()
        }
    }

    fn parse_infix_expression(p: &mut Parser, left_val: Expression) -> Option<ast::Expression> {
        let left = Box::new(left_val);
        let operator = p.curr_token.clone();
        let precedence = p.curr_precedence();
        p.next_token();
        let right = Box::new(p.parse_expression(precedence)?);
        return Some(Expression::InfixOperation {
            left,
            operator,
            right,
        });
    }

    fn infix_parse_fns(&mut self, tok: &Token) -> InfixParseFunc {
        todo!()
    }
    fn curr_token_is(&self, token_type: &Token) -> bool {
        discriminant(&self.curr_token) == discriminant(token_type)
    }
    fn peek_token_is(&self, token_type: &Token) -> bool {
        discriminant(&self.peek_token) == discriminant(token_type)
    }
    pub fn parse_program(&mut self) -> ast::Program {
        let mut statements: Vec<ast::Statement> = Vec::new();
        while !self.curr_token_is(&Token::Eof) {
            if let Some(sttmnt) = self.parse_statement() {
                statements.push(sttmnt);
            }
            self.next_token()
        }
        return ast::Program { statements };
    }

    fn next_token(&mut self) {
        self.curr_token = std::mem::replace(&mut self.peek_token, self.lexer.next_token());
    }

    fn parse_statement(&mut self) -> Option<Statement> {
        match self.curr_token {
            Token::Let => self.parse_let_statement(),
            Token::Return => self.parse_return_statement(),
            _ => self.parse_expression_statement(),
        }
    }
    fn parse_expression(&mut self, precedence: Precedence) -> Option<Expression> {
        todo!()
    }

    fn parse_expression_statement(&mut self) -> Option<Statement> {
        let val = self.parse_expression(Precedence::Lowest)?;
        if self.peek_token_is(&Token::Semicolon) {
            self.next_token()
        }
        return Some(Statement::Expression(val));
    }

    fn parse_let_statement(&mut self) -> Option<Statement> {
        let name = if let Token::Ident(name) = &self.peek_token {
            name.clone()
        } else {
            self.peek_error(&token::empty_ident());
            return None;
        };
        self.next_token();

        if let Token::Assign = self.peek_token {
            self.next_token()
        } else {
            return None;
        };
        let val = self.parse_expression(Precedence::Lowest)?;
        if self.peek_token_is(&Token::Semicolon) {
            self.next_token()
        }
        return Some(Statement::Let {
            identifier: ast::Ident { name },
            value: val,
        });
    }

    fn parse_return_statement(&mut self) -> Option<Statement> {
        self.next_token();
        let val = self.parse_expression(Precedence::Lowest)?;
        if self.peek_token_is(&Token::Semicolon) {
            self.next_token()
        }
        return Some(Statement::Return(val));
    }

    fn expect_peek_advance(&mut self, t: &Token) -> bool {
        if self.peek_token_is(t) {
            self.next_token();
            return true;
        } else {
            self.peek_error(t);
            return false;
        }
    }

    fn peek_error(&mut self, t: &Token) {
        self.errors.push(format!(
            "Expected next token to be {}, got {}",
            t, self.peek_token
        ))
    }

    fn peek_precedence(&self) -> Precedence {
        return precedences(&self.peek_token);
    }

    fn curr_precedence(&self) -> Precedence {
        return precedences(&self.curr_token);
    }

    pub fn errors(&self) -> &Vec<String> {
        &self.errors
    }
}
