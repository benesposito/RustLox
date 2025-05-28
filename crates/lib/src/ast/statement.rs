pub mod declaration;

use super::expression::{Expression, Value};
use crate::ast::{ParseContext, ParseErrorKind, ParseResult, ShouldSynchronize};
use crate::environment::Environment;
use crate::evaluator::RuntimeError;
use declaration::Declaration;

use lexer::{Token, tokens::FixedToken};

#[derive(Debug)]
pub enum Statement {
    Expression(Expression),
    Block(Vec<Box<Declaration>>),
    Print(Expression),
    If {
        conditional: Expression,
        then: Box<Statement>,
        else_: Option<Box<Statement>>,
    },
}

impl Statement {
    fn parse<T: Iterator<Item = Token>>(
        parse_context: &mut ParseContext<ParseErrorKind, T>,
    ) -> ParseResult<Self> {
        statement(parse_context)
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
            Statement::If { conditional, then, else_ } => {
                let Value::Boolean(conditional) = conditional.evaluate(environment)? else {
                    return Err(RuntimeError::TypeError);
                };

                if conditional {
                    then.evaluate(environment)
                } else {
                    else_.as_ref().map_or(Ok(()), |e| e.evaluate(environment))
                }
            }
            Statement::Print(expression) => {
                println!("{}", expression.evaluate(environment)?);
                Ok(())
            }
        }
    }
}

fn statement<T: Iterator<Item = Token>>(
    parse_context: &mut ParseContext<ParseErrorKind, T>,
) -> ParseResult<Statement> {
    match parse_context.tokens().peek().expect("Expected tokens") {
        Token::FixedToken(FixedToken::LeftBrace) => {
            parse_context.tokens().next();

            let mut declarations: Vec<Declaration> = Vec::new();

            while !matches!(
                parse_context.tokens().peek(),
                Some(Token::FixedToken(FixedToken::RightBrace))
            ) {
                declarations.push(Declaration::parse(parse_context)?);
            }

            parse_context.tokens().next();

            Ok(Statement::Block(
                declarations
                    .into_iter()
                    .map(|d| Box::new(d))
                    .collect::<Vec<_>>(),
            ))
        }
        Token::FixedToken(FixedToken::If) => {
            parse_context.tokens().next();

            let Some(Token::FixedToken(FixedToken::LeftParenthesis)) =
                parse_context.tokens().next()
            else {
                parse_context.record_error(ParseErrorKind::UnexpectedToken);
                return Err(ShouldSynchronize::Yes);
            };

            let conditional = Expression::parse(parse_context)?;

            let Some(Token::FixedToken(FixedToken::RightParenthesis)) =
                parse_context.tokens().next()
            else {
                parse_context.record_error(ParseErrorKind::UnexpectedToken);
                return Err(ShouldSynchronize::Yes);
            };

            let then = Statement::parse(parse_context)?;

            let else_ = match parse_context.tokens().next() {
                Some(Token::FixedToken(FixedToken::Else)) => Some(Statement::parse(parse_context)?),
                _ => None,
            };

            return Ok(Statement::If {
                conditional,
                then: Box::new(then),
                else_: else_.map(Box::new),
            });
        }
        Token::FixedToken(FixedToken::Print) => {
            parse_context.tokens().next();

            let statement = Statement::Print(Expression::parse(parse_context)?);
            match parse_context.tokens().next() {
                Some(Token::FixedToken(FixedToken::Semicolon)) => Ok(statement),
                _ => {
                    parse_context.record_error(ParseErrorKind::ExpectedSemicolon);
                    Err(ShouldSynchronize::Yes)
                }
            }
        }
        _ => {
            let statement = Statement::Expression(Expression::parse(parse_context)?);
            match parse_context.tokens().next() {
                Some(Token::FixedToken(FixedToken::Semicolon)) => Ok(statement),
                _ => {
                    parse_context.record_error(ParseErrorKind::ExpectedSemicolon);
                    Err(ShouldSynchronize::Yes)
                }
            }
        }
    }
}

use std::fmt;

impl fmt::Display for Statement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Statement::Expression(expression) => write!(f, "{}", expression),
            Statement::Block(statements) => write!(f, "(block {:?})", statements),
            Statement::If {
                conditional,
                then,
                else_,
            } => write!(f, "(if {conditional} {then} {else_:?})"),
            Statement::Print(expression) => write!(f, "(print {})", expression),
        }
    }
}
