use super::{BinaryOperator, Expression, UnaryOperator, Value};
use crate::ast::{ParseErrorKind, ParseResult};
use crate::lexer::Token;

use std::iter::Peekable;

pub fn expression(tokens: &mut Peekable<impl Iterator<Item = Token>>) -> ParseResult<Expression> {
    logical_or(tokens)
}

fn logical_or(tokens: &mut Peekable<impl Iterator<Item = Token>>) -> ParseResult<Expression> {
    let mut expression = logical_and(tokens)?;

    loop {
        let Some(token) = tokens.peek() else {
            return Ok(expression);
        };

        let operator = match token {
            Token::Or => BinaryOperator::Or,
            _ => return Ok(expression),
        };

        tokens.next();

        let right = logical_and(tokens)?;
        expression = Expression::Binary(Box::new(expression), operator, Box::new(right));
    }
}

fn logical_and(tokens: &mut Peekable<impl Iterator<Item = Token>>) -> ParseResult<Expression> {
    let mut expression = equality(tokens)?;

    loop {
        let Some(token) = tokens.peek() else {
            return Ok(expression);
        };

        let operator = match token {
            Token::And => BinaryOperator::And,
            _ => return Ok(expression),
        };

        tokens.next();

        let right = equality(tokens)?;
        expression = Expression::Binary(Box::new(expression), operator, Box::new(right));
    }
}

fn equality(tokens: &mut Peekable<impl Iterator<Item = Token>>) -> ParseResult<Expression> {
    let mut expression = comparison(tokens)?;

    loop {
        let Some(token) = tokens.peek() else {
            return Ok(expression);
        };

        let operator = match token {
            Token::EqualEqual => BinaryOperator::Equality,
            Token::BangEqual => BinaryOperator::Inequality,
            _ => return Ok(expression),
        };

        tokens.next();

        let right = comparison(tokens)?;
        expression = Expression::Binary(Box::new(expression), operator, Box::new(right));
    }
}

fn comparison(tokens: &mut Peekable<impl Iterator<Item = Token>>) -> ParseResult<Expression> {
    let mut expression = term(tokens)?;

    loop {
        let Some(token) = tokens.peek() else {
            return Ok(expression);
        };

        let operator = match token {
            Token::Greater => BinaryOperator::GreaterThan,
            Token::GreaterEqual => BinaryOperator::GreaterThanOrEqualTo,
            Token::Less => BinaryOperator::LessThan,
            Token::LessEqual => BinaryOperator::LessThanOrEqualTo,
            _ => return Ok(expression),
        };

        tokens.next();

        let right = term(tokens)?;
        expression = Expression::Binary(Box::new(expression), operator, Box::new(right));
    }
}

fn term(tokens: &mut Peekable<impl Iterator<Item = Token>>) -> ParseResult<Expression> {
    let mut expression = factor(tokens)?;

    loop {
        let Some(token) = tokens.peek() else {
            return Ok(expression);
        };

        let operator = match token {
            Token::Plus => BinaryOperator::Addition,
            Token::Minus => BinaryOperator::Subtraction,
            _ => return Ok(expression),
        };

        tokens.next();

        let right = factor(tokens)?;
        expression = Expression::Binary(Box::new(expression), operator, Box::new(right));
    }
}

fn factor(tokens: &mut Peekable<impl Iterator<Item = Token>>) -> ParseResult<Expression> {
    let mut expression = unary(tokens)?;

    loop {
        let Some(token) = tokens.peek() else {
            return Ok(expression);
        };

        let operator = match token {
            Token::Asterisk => BinaryOperator::Multiplication,
            Token::ForwardSlash => BinaryOperator::Division,
            _ => return Ok(expression),
        };

        tokens.next();

        let right = unary(tokens)?;
        expression = Expression::Binary(Box::new(expression), operator, Box::new(right));
    }
}

fn unary(tokens: &mut Peekable<impl Iterator<Item = Token>>) -> ParseResult<Expression> {
    let operator = match tokens.peek().expect("Expected unary or primary expression") {
        Token::Minus => UnaryOperator::Negate,
        Token::Bang => UnaryOperator::Not,
        _ => return primary(tokens),
    };

    tokens.next();
    Ok(Expression::Unary(operator, Box::new(unary(tokens)?)))
}

fn primary(tokens: &mut Peekable<impl Iterator<Item = Token>>) -> ParseResult<Expression> {
    match tokens.next().expect("Expected primary expression") {
        Token::LeftParenthesis => {
            let expression = expression(tokens)?;

            match tokens.next() {
                Some(token) => match token {
                    Token::RightParenthesis => Ok(Expression::Grouping(Box::new(expression))),
                    _ => panic!("Expected right parenthesis, instead got {:?}", token),
                },
                None => Err(ParseErrorKind::UnmatchedParenthesis),
            }
        }
        Token::Identifier(name) => Ok(Expression::Variable(name)),
        Token::NumericLiteral(value) => Ok(Expression::Value(Value::Numeric(value))),
        Token::StringLiteral(value) => Ok(Expression::Value(Value::Str(value.clone()))),
        Token::True => Ok(Expression::Value(Value::Boolean(true))),
        Token::False => Ok(Expression::Value(Value::Boolean(false))),
        Token::Nil => Ok(Expression::Value(Value::Nil)),
        _ => Err(ParseErrorKind::ExpectedPrimaryExpressionBefore),
    }
}
