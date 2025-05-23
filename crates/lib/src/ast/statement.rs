use super::expression;
use super::{ParseErrorKind, ParseResult};
use crate::environment::Environment;
use crate::evaluator::RuntimeError;
use expression::Expression;

use lexer::{Token, tokens::FixedToken};

use std::iter::Peekable;

#[derive(Debug)]
pub enum Statement {
    Expression(Expression),
    Block(Vec<Box<Statement>>),
    Print(Expression),
    VariableDeclaration(String),
    VariableDefinition(String, Expression),
}

impl Statement {
    pub fn parse(tokens: &mut Peekable<impl Iterator<Item = Token>>) -> ParseResult<Self> {
        statement(tokens)
    }

    pub fn evaluate(&self, environment: &mut Environment) -> Result<(), RuntimeError> {
        match self {
            Statement::Expression(expression) => expression.evaluate(environment).map(|_| ()),
            Statement::Block(statements) => {
                let mut environment = Environment::inner(environment);
                for statement in statements {
                    statement.evaluate(&mut environment)?
                }
                Ok(())
            }
            Statement::Print(expression) => {
                println!("{}", expression.evaluate(environment)?);
                Ok(())
            }
            Statement::VariableDeclaration(identifier) => {
                environment.declare_variable(identifier);
                Ok(())
            }
            Statement::VariableDefinition(identifier, expression) => {
                let value = expression.evaluate(environment)?;
                environment.define_variable(identifier, value);
                Ok(())
            }
        }
    }
}

fn statement(tokens: &mut Peekable<impl Iterator<Item = Token>>) -> ParseResult<Statement> {
    let statement = match tokens.peek().expect("Expected tokens") {
        Token::FixedToken(FixedToken::LeftBrace) => {
            tokens.next();

            let mut statements: Vec<Box<Statement>> = Vec::new();

            while !matches!(
                tokens.peek(),
                Some(Token::FixedToken(FixedToken::RightBrace))
            ) {
                statements.push(Box::new(statement(tokens)?));
            }

            tokens.next();

            return Ok(Statement::Block(statements));
        }
        Token::FixedToken(FixedToken::Print) => {
            tokens.next();
            Statement::Print(Expression::parse(tokens)?)
        }
        Token::FixedToken(FixedToken::Var) => {
            tokens.next();

            let identifier = match tokens.next() {
                Some(Token::Identifier(identifier)) => identifier,
                _ => return Err(ParseErrorKind::ExpectedIdentifier),
            };

            match tokens.peek() {
                Some(Token::FixedToken(FixedToken::Equal)) => {
                    tokens.next();
                    Statement::VariableDefinition(identifier.name, Expression::parse(tokens)?)
                }
                _ => Statement::VariableDeclaration(identifier.name),
            }
        }
        _ => Statement::Expression(Expression::parse(tokens)?),
    };

    match tokens.next() {
        Some(Token::FixedToken(FixedToken::Semicolon)) => Ok(statement),
        _ => {
            loop {
                match tokens.next() {
                    Some(Token::FixedToken(FixedToken::Semicolon)) | None => break,
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
            Statement::Block(statements) => write!(f, "(block {:?})", statements),
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
