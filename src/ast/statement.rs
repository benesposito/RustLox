use super::expression::{Expression, Value};
use super::{ParseErrorKind, ParseResult};
use crate::environment::Environment;
use crate::lexer::Token;

use std::iter::Peekable;

pub enum Statement {
    Expression(Expression),
    Print(Expression),
    VariableDeclaration(String),
    VariableDefinition(String, Expression),
}

impl Statement {
    pub fn parse(tokens: &mut Peekable<impl Iterator<Item = Token>>) -> ParseResult<Self> {
        statement(tokens)
    }

    pub fn evaluate(&self, environment: &mut Environment) -> Option<Value> {
        match self {
            Statement::Expression(expression) => Some(expression.evaluate(environment)),
            Statement::Print(expression) => {
                println!("{}", expression.evaluate(environment));
                None
            }
            Statement::VariableDeclaration(identifier) => {
                environment.declare_variable(identifier);
                None
            }
            Statement::VariableDefinition(identifier, expression) => {
                let value = expression.evaluate(environment);
                environment.define_variable(identifier, value);
                None
            }
        }
    }
}

fn statement(tokens: &mut Peekable<impl Iterator<Item = Token>>) -> ParseResult<Statement> {
    let statement = match tokens.peek().expect("Expected tokens") {
        Token::Print => {
            tokens.next();
            Statement::Print(Expression::parse(tokens)?)
        }
        Token::Var => {
            tokens.next();

            let variable_name = match tokens.next() {
                Some(Token::Identifier(identifier)) => identifier,
                _ => return Err(ParseErrorKind::ExpectedIdentifier),
            };

            match tokens.peek() {
                Some(Token::Equal) => {
                    tokens.next();
                    Statement::VariableDefinition(variable_name, Expression::parse(tokens)?)
                }
                _ => Statement::VariableDeclaration(variable_name),
            }
        }
        _ => Statement::Expression(Expression::parse(tokens)?),
    };

    match tokens.next() {
        Some(Token::Semicolon) => Ok(statement),
        _ => {
            loop {
                match tokens.next() {
                    Some(Token::Semicolon) | None => break,
                    _ => (),
                };
            }

            Err(ParseErrorKind::ExpectedSemicolon)
        }
    }
}

use std::fmt;

impl fmt::Display for Statement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Statement::Expression(expression) => write!(f, "{}", expression),
            Statement::Print(expression) => write!(f, "(print {})", expression),
            Statement::VariableDeclaration(identifier) => {
                write!(f, "(declare-variable {})", identifier)
            }
            Statement::VariableDefinition(identifier, initial_value) => {
                write!(f, "(define-variable {} {})", identifier, initial_value)
            }
        }
    }
}
