use std::{collections::HashMap, fmt::Display};

use crate::token::Token;

pub trait Node: Display {}

// pub struct Error {}
// impl Display for Error {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         write!(f, "")
//     }
// }
// impl Node for Error {}

pub enum Statement {
    Let {
        identifier: Ident,
        value: Expression,
    },
    Return(Expression),
    Expression(Expression),
    Block(Vec<Statement>),
}
impl Display for Statement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Statement::Let { identifier, value } => write!(f, "let {} = {};\n", identifier, value),
            Statement::Return(val) => write!(f, "return {};\n", val),
            Statement::Expression(val) => write!(f, "{};\n", val),
            Statement::Block(vals) => {
                write!(f, "{{\n")?;
                for val in vals {
                    write!(f, "\t{}\n", val)?
                }
                write!(f, "}}\n")?;

                Ok(())
            }
        }
    }
}
impl Node for Statement {}
pub enum Expression {
    Identifier(Ident),
    IntegerLiteral(i64),
    PrefixOperation {
        operator: Token,
        right: Box<Expression>,
    },
    InfixOperation {
        operator: Token,
        left: Box<Expression>,
        right: Box<Expression>,
    },
    BooleanLiteral(bool),
    If {
        condition: Box<Expression>,
        consequence: Box<Statement>,
        alternative: Option<Box<Statement>>,
    },
    FunctionLiteral {
        parameters: Vec<Ident>,
        body: Box<Statement>,
    },
    FunctionCall {
        function: Box<Expression>,
        arguments: Vec<Expression>,
    },
    StringLiteral(String),
    ArrayLiteral(Vec<Expression>),
    Index {
        left: Box<Expression>,
        index: Box<Expression>,
    },
    HashLiteral(HashMap<Box<Expression>, Box<Expression>>),
    Error,
}
impl Display for Expression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Expression::Identifier(val) => write!(f, "{}", val),
            Expression::IntegerLiteral(val) => write!(f, "{}", val),
            Expression::PrefixOperation { operator, right } => write!(f, "{}{}", operator, right),
            Expression::InfixOperation {
                operator,
                left,
                right,
            } => write!(f, "{} {} {}", left, operator, right),
            Expression::BooleanLiteral(val) => write!(f, "{}", val),
            Expression::If {
                condition,
                consequence,
                alternative,
            } => {
                write!(f, "if ({}) {}", condition, consequence)?;
                if let Some(alt) = alternative {
                    write!(f, "else {}", alt)?;
                }
                Ok(())
            }

            Expression::FunctionLiteral { parameters, body } => {
                write!(
                    f,
                    "fn ({}) {}",
                    parameters
                        .into_iter()
                        .map(|p| p.name.clone())
                        .collect::<Vec<_>>()
                        .join(", "),
                    body
                )
            }

            Expression::FunctionCall {
                function,
                arguments,
            } => write!(
                f,
                "{}({})",
                function,
                arguments
                    .into_iter()
                    .map(|a| format!("{}", a))
                    .collect::<Vec<_>>()
                    .join(", ")
            ),
            Expression::StringLiteral(val) => write!(f, "{}", val),
            Expression::ArrayLiteral(values) => write!(
                f,
                "[{}]",
                values
                    .into_iter()
                    .map(|a| format!("{}", a))
                    .collect::<Vec<_>>()
                    .join(", ")
            ),
            Expression::Index { left, index } => write!(f, "{}[{}]", left, index),
            Expression::HashLiteral(map) => write!(
                f,
                "[{{{}}}",
                map.into_iter()
                    .map(|(key, val)| format!("{}:{}", key, val))
                    .collect::<Vec<_>>()
                    .join(", ")
            ),
            Expression::Error => Ok(()),
        }
    }
}
impl Node for Expression {}

pub struct Ident {
    pub(crate) name: String,
}
impl Display for Ident {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}

pub struct Program {
    pub(crate) statements: Vec<Statement>,
}
impl Display for Program {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for statement in &self.statements {
            write!(f, "{}", statement)?
        }
        Ok(())
    }
}
impl Node for Program {}
