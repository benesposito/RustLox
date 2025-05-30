use lexer::{Token, tokens::FixedToken};

use crate::grammar::*;
use crate::parser::*;

impl Statement {
    pub fn parse<T: Iterator<Item = Token>>(
        parse_context: &mut ParseContext<ParseErrorKind, T>,
    ) -> ParseResult<Self> {
        statement(parse_context)
    }
}

impl Block {
    fn parse<T: Iterator<Item = Token>>(
        parse_context: &mut ParseContext<ParseErrorKind, T>,
    ) -> ParseResult<Self> {
        parse_context.tokens().next();

        let mut declarations: Vec<Declaration> = Vec::new();

        while !matches!(
            parse_context.tokens().peek(),
            Some(Token::FixedToken(FixedToken::RightBrace))
        ) {
            declarations.push(Declaration::parse(parse_context)?);
        }

        parse_context.tokens().next();

        Ok(Self {
            statements: declarations.into_iter().collect::<Vec<_>>(),
        })
    }
}

fn statement<T: Iterator<Item = Token>>(
    parse_context: &mut ParseContext<ParseErrorKind, T>,
) -> ParseResult<Statement> {
    match parse_context.tokens().peek().expect("Expected tokens") {
        Token::FixedToken(FixedToken::LeftBrace) => {
            Block::parse(parse_context).map(Statement::Block)
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

            return Ok(Statement::IfStatement {
                conditional,
                then: Box::new(then),
                else_: else_.map(Box::new),
            });
        }
        Token::FixedToken(FixedToken::Print) => {
            parse_context.tokens().next();

            let statement = Statement::PrintStatement(Expression::parse(parse_context)?);
            match parse_context.tokens().next() {
                Some(Token::FixedToken(FixedToken::Semicolon)) => Ok(statement),
                _ => {
                    parse_context.record_error(ParseErrorKind::ExpectedSemicolon);
                    Err(ShouldSynchronize::Yes)
                }
            }
        }
        _ => {
            let statement = Statement::ExpressionStatement(Expression::parse(parse_context)?);
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
            Statement::ExpressionStatement(expression) => write!(f, "{}", expression),
            Statement::IfStatement {
                conditional,
                then,
                else_,
            } => write!(f, "(if {conditional} {then} {else_:?})"),
            Statement::PrintStatement(expression) => write!(f, "(print {})", expression),
            Statement::Block(statements) => write!(f, "(block {:?})", statements),
        }
    }
}
