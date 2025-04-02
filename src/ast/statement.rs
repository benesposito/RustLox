use super::expression::Expression;
use super::{ParseErrorKind, ParseResult};
use crate::lexer::Token;

use std::iter::Peekable;

pub enum Statement {
    Expression(Expression),
    Print(Expression),
}

impl Statement {
    pub fn parse(tokens: &mut Peekable<impl Iterator<Item = Token>>) -> ParseResult<Self> {
        statement(tokens)
    }

    pub fn evaluate(&self) -> Option<super::expression::Value> {
        match self {
            Statement::Expression(expression) => Some(expression.evaluate()),
            Statement::Print(expression) => {
                println!("{}", expression.evaluate());
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
