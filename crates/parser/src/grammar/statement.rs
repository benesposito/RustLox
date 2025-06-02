use lexer::{Token, tokens::FixedToken};

use crate::grammar::*;
use crate::parser::*;

impl Statement {
    pub fn parse<T: Iterator<Item = Token>>(
        parse_context: &mut ParseContext<T>,
    ) -> ParseResult<Self> {
        statement(parse_context)
    }
}

impl Block {
    fn parse<T: Iterator<Item = Token>>(parse_context: &mut ParseContext<T>) -> ParseResult<Self> {
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

impl ForLoopInitializer {
    pub fn parse<T: Iterator<Item = Token>>(
        parse_context: &mut ParseContext<T>,
    ) -> ParseResult<Self> {
        match parse_context.tokens().peek().expect("Expected tokens") {
            Token::FixedToken(FixedToken::Var) => Ok(ForLoopInitializer::Declaration(
                VariableDeclaration::parse(parse_context)?,
            )),
            _ => {
                parse_context.tokens().next().unwrap();
                parse_context.record_error(ParseErrorKind::UnexpectedToken);
                return Err(ShouldSynchronize::Yes);
            }
        }
    }
}

fn statement<T: Iterator<Item = Token>>(
    parse_context: &mut ParseContext<T>,
) -> ParseResult<Statement> {
    match parse_context.tokens().peek().expect("Expected tokens") {
        Token::FixedToken(FixedToken::If) => {
            parse_context.tokens().next();

            parse_context.match_token(FixedToken::LeftParenthesis)?;
            let condition = Expression::parse(parse_context)?;
            parse_context.match_token(FixedToken::RightParenthesis)?;

            let then = Statement::parse(parse_context)?;

            let else_ = match parse_context.tokens().next() {
                Some(Token::FixedToken(FixedToken::Else)) => Some(Statement::parse(parse_context)?),
                _ => None,
            };

            return Ok(Statement::IfStatement {
                condition,
                then: Box::new(then),
                else_: else_.map(Box::new),
            });
        }
        Token::FixedToken(FixedToken::For) => {
            parse_context.tokens().next();

            println!("token: {:?}", parse_context.tokens().peek());

            parse_context.match_token(FixedToken::LeftParenthesis)?;

            println!("token: {:?}", parse_context.tokens().peek());

            let initializer = ForLoopInitializer::parse(parse_context)?;

            let condition = match parse_context.tokens().peek() {
                Some(Token::FixedToken(FixedToken::Semicolon)) => None,
                _ => Some(Expression::parse(parse_context)?),
            };

            parse_context.match_token(FixedToken::Semicolon)?;

            let expression = match parse_context.tokens().peek() {
                Some(Token::FixedToken(FixedToken::Semicolon)) => None,
                _ => Some(Expression::parse(parse_context)?),
            };

            parse_context.match_token(FixedToken::RightParenthesis)?;

            let body = Statement::parse(parse_context)?;

            return Ok(Statement::ForStatement {
                initializer,
                condition,
                expression,
                body: Box::new(body),
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
        Token::FixedToken(FixedToken::While) => {
            parse_context.tokens().next();
            parse_context.match_token(FixedToken::LeftParenthesis)?;

            let condition = Expression::parse(parse_context)?;

            parse_context.match_token(FixedToken::RightParenthesis)?;

            let body = Statement::parse(parse_context)?;

            return Ok(Statement::WhileStatement {
                condition,
                body: Box::new(body),
            });
        }
        Token::FixedToken(FixedToken::LeftBrace) => {
            Block::parse(parse_context).map(Statement::Block)
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
            Statement::ForStatement {
                initializer,
                condition,
                expression,
                body,
            } => write!(
                f,
                "(for {initializer:?} {condition:?} {expression:?} {body})"
            ),
            Statement::IfStatement {
                condition,
                then,
                else_,
            } => write!(f, "(if {condition} {then} {else_:?})"),
            Statement::PrintStatement(expression) => write!(f, "(print {})", expression),
            Statement::WhileStatement { condition, body } => {
                write!(f, "(while {condition} {body})")
            }
            Statement::Block(statements) => write!(f, "(block {:?})", statements),
        }
    }
}
