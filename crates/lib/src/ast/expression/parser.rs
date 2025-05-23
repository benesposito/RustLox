use super::{BinaryOperator, Expression, UnaryOperator, Value};
use crate::ast::{ParseErrorKind, ParseResult};

use lexer::{Token, tokens::FixedToken};

use std::iter::Peekable;

pub fn expression(tokens: &mut Peekable<impl Iterator<Item = Token>>) -> ParseResult<Expression> {
    logical_or(tokens)
}

fn logical_or(tokens: &mut Peekable<impl Iterator<Item = Token>>) -> ParseResult<Expression> {
    let mut expression = logical_and(tokens)?;

    loop {
        let Some(Token::FixedToken(token)) = tokens.peek() else {
            return Ok(expression);
        };

        let operator = match token {
            FixedToken::Or => BinaryOperator::Or,
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
        let Some(Token::FixedToken(token)) = tokens.peek() else {
            return Ok(expression);
        };

        let operator = match token {
            FixedToken::And => BinaryOperator::And,
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
        let Some(Token::FixedToken(token)) = tokens.peek() else {
            return Ok(expression);
        };

        let operator = match token {
            FixedToken::EqualEqual => BinaryOperator::Equality,
            FixedToken::BangEqual => BinaryOperator::Inequality,
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
        let Some(Token::FixedToken(token)) = tokens.peek() else {
            return Ok(expression);
        };

        let operator = match token {
            FixedToken::Greater => BinaryOperator::GreaterThan,
            FixedToken::GreaterEqual => BinaryOperator::GreaterThanOrEqualTo,
            FixedToken::Less => BinaryOperator::LessThan,
            FixedToken::LessEqual => BinaryOperator::LessThanOrEqualTo,
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
        let Some(Token::FixedToken(token)) = tokens.peek() else {
            return Ok(expression);
        };

        let operator = match token {
            FixedToken::Plus => BinaryOperator::Addition,
            FixedToken::Minus => BinaryOperator::Subtraction,
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
        let Some(Token::FixedToken(token)) = tokens.peek() else {
            return Ok(expression);
        };

        let operator = match token {
            FixedToken::Asterisk => BinaryOperator::Multiplication,
            FixedToken::ForwardSlash => BinaryOperator::Division,
            _ => return Ok(expression),
        };

        tokens.next();

        let right = unary(tokens)?;
        expression = Expression::Binary(Box::new(expression), operator, Box::new(right));
    }
}

fn unary(tokens: &mut Peekable<impl Iterator<Item = Token>>) -> ParseResult<Expression> {
    let operator = match tokens.peek().expect("Expected unary or primary expression") {
        Token::FixedToken(FixedToken::Minus) => UnaryOperator::Negate,
        Token::FixedToken(FixedToken::Bang) => UnaryOperator::Not,
        _ => return call(tokens),
    };

    tokens.next();
    Ok(Expression::Unary(operator, Box::new(unary(tokens)?)))
}

fn call(tokens: &mut Peekable<impl Iterator<Item = Token>>) -> ParseResult<Expression> {
    let callable = primary(tokens)?;

    let Some(Token::FixedToken(FixedToken::LeftParenthesis)) = tokens.peek() else {
        return Ok(callable);
    };

    tokens.next();

    let mut arguments: Vec<Expression> = Vec::new();

    if let Some(Token::FixedToken(FixedToken::RightParenthesis)) = tokens.peek() {
        tokens.next();
        return Ok(Expression::FunctionCall(Box::new(callable), arguments));
    };

    loop {
        arguments.push(expression(tokens)?);

        match tokens.next() {
            Some(Token::FixedToken(FixedToken::Comma)) => (),
            Some(Token::FixedToken(FixedToken::RightParenthesis)) => break,
            _ => return Err(ParseErrorKind::UnexpectedToken),
        }
    }

    Ok(Expression::FunctionCall(Box::new(callable), arguments))
}

fn primary(tokens: &mut Peekable<impl Iterator<Item = Token>>) -> ParseResult<Expression> {
    match tokens.next().expect("Expected primary expression") {
        Token::FixedToken(FixedToken::LeftParenthesis) => {
            let expression = expression(tokens)?;

            match tokens.next() {
                Some(token) => match token {
                    Token::FixedToken(FixedToken::RightParenthesis) => {
                        Ok(Expression::Grouping(Box::new(expression)))
                    }
                    _ => panic!("Expected right parenthesis, instead got {:?}", token),
                },
                None => Err(ParseErrorKind::UnmatchedParenthesis),
            }
        }
        Token::Identifier(identifier) => Ok(Expression::Variable(identifier.name)),
        Token::NumericLiteral(literal) => Ok(Expression::Value(Value::Numeric(literal.value))),
        Token::StringLiteral(literal) => Ok(Expression::Value(Value::Str(literal.value.clone()))),
        Token::FixedToken(FixedToken::True) => Ok(Expression::Value(Value::Boolean(true))),
        Token::FixedToken(FixedToken::False) => Ok(Expression::Value(Value::Boolean(false))),
        Token::FixedToken(FixedToken::Nil) => Ok(Expression::Value(Value::Nil)),
        _ => Err(ParseErrorKind::ExpectedPrimaryExpression),
    }
}
