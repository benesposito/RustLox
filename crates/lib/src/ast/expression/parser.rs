use super::*;
use crate::ast::ParseErrorKind;

use lexer::{tokens::FixedToken, Token};

pub fn expression<T: Iterator<Item = Token>>(
    parse_context: &mut ParseContext<ParseErrorKind, T>,
) -> ParseResult<Expression> {
    logical_or(parse_context)
}

fn logical_or<T: Iterator<Item = Token>>(
    parse_context: &mut ParseContext<ParseErrorKind, T>,
) -> ParseResult<Expression> {
    let mut expression = logical_and(parse_context)?;

    loop {
        let Some(Token::FixedToken(token)) = parse_context.tokens().peek() else {
            return Ok(expression);
        };

        let operator = match token {
            FixedToken::Or => BinaryOperator::Or,
            _ => return Ok(expression),
        };

        parse_context.tokens().next();

        let right = logical_and(parse_context)?;
        expression = Expression::Binary(Box::new(expression), operator, Box::new(right));
    }
}

fn logical_and<T: Iterator<Item = Token>>(
    parse_context: &mut ParseContext<ParseErrorKind, T>,
) -> ParseResult<Expression> {
    let mut expression = equality(parse_context)?;

    loop {
        let Some(Token::FixedToken(token)) = parse_context.tokens().peek() else {
            return Ok(expression);
        };

        let operator = match token {
            FixedToken::And => BinaryOperator::And,
            _ => return Ok(expression),
        };

        parse_context.tokens().next();

        let right = equality(parse_context)?;
        expression = Expression::Binary(Box::new(expression), operator, Box::new(right));
    }
}

fn equality<T: Iterator<Item = Token>>(
    parse_context: &mut ParseContext<ParseErrorKind, T>,
) -> ParseResult<Expression> {
    let mut expression = comparison(parse_context)?;

    loop {
        let Some(Token::FixedToken(token)) = parse_context.tokens().peek() else {
            return Ok(expression);
        };

        let operator = match token {
            FixedToken::EqualEqual => BinaryOperator::Equality,
            FixedToken::BangEqual => BinaryOperator::Inequality,
            _ => return Ok(expression),
        };

        parse_context.tokens().next();

        let right = comparison(parse_context)?;
        expression = Expression::Binary(Box::new(expression), operator, Box::new(right));
    }
}

fn comparison<T: Iterator<Item = Token>>(
    parse_context: &mut ParseContext<ParseErrorKind, T>,
) -> ParseResult<Expression> {
    let mut expression = term(parse_context)?;

    loop {
        let Some(Token::FixedToken(token)) = parse_context.tokens().peek() else {
            return Ok(expression);
        };

        let operator = match token {
            FixedToken::Greater => BinaryOperator::GreaterThan,
            FixedToken::GreaterEqual => BinaryOperator::GreaterThanOrEqualTo,
            FixedToken::Less => BinaryOperator::LessThan,
            FixedToken::LessEqual => BinaryOperator::LessThanOrEqualTo,
            _ => return Ok(expression),
        };

        parse_context.tokens().next();

        let right = term(parse_context)?;
        expression = Expression::Binary(Box::new(expression), operator, Box::new(right));
    }
}

fn term<T: Iterator<Item = Token>>(
    parse_context: &mut ParseContext<ParseErrorKind, T>,
) -> ParseResult<Expression> {
    let mut expression = factor(parse_context)?;

    loop {
        let Some(Token::FixedToken(token)) = parse_context.tokens().peek() else {
            return Ok(expression);
        };

        let operator = match token {
            FixedToken::Plus => BinaryOperator::Addition,
            FixedToken::Minus => BinaryOperator::Subtraction,
            _ => return Ok(expression),
        };

        parse_context.tokens().next();

        let right = factor(parse_context)?;
        expression = Expression::Binary(Box::new(expression), operator, Box::new(right));
    }
}

fn factor<T: Iterator<Item = Token>>(
    parse_context: &mut ParseContext<ParseErrorKind, T>,
) -> ParseResult<Expression> {
    let mut expression = unary(parse_context)?;

    loop {
        let Some(Token::FixedToken(token)) = parse_context.tokens().peek() else {
            return Ok(expression);
        };

        let operator = match token {
            FixedToken::Asterisk => BinaryOperator::Multiplication,
            FixedToken::ForwardSlash => BinaryOperator::Division,
            _ => return Ok(expression),
        };

        parse_context.tokens().next();

        let right = unary(parse_context)?;
        expression = Expression::Binary(Box::new(expression), operator, Box::new(right));
    }
}

fn unary<T: Iterator<Item = Token>>(
    parse_context: &mut ParseContext<ParseErrorKind, T>,
) -> ParseResult<Expression> {
    let operator = match parse_context
        .tokens()
        .peek()
        .expect("Expected unary or primary expression")
    {
        Token::FixedToken(FixedToken::Minus) => UnaryOperator::Negate,
        Token::FixedToken(FixedToken::Bang) => UnaryOperator::Not,
        _ => return call(parse_context),
    };

    parse_context.tokens().next();
    Ok(Expression::Unary(operator, Box::new(unary(parse_context)?)))
}

fn call<T: Iterator<Item = Token>>(
    parse_context: &mut ParseContext<ParseErrorKind, T>,
) -> ParseResult<Expression> {
    let callable = primary(parse_context)?;

    let Some(Token::FixedToken(FixedToken::LeftParenthesis)) = parse_context.tokens().peek() else {
        return Ok(callable);
    };

    parse_context.tokens().next();

    let mut arguments: Vec<Expression> = Vec::new();

    if let Some(Token::FixedToken(FixedToken::RightParenthesis)) = parse_context.tokens().peek() {
        parse_context.tokens().next();
        return Ok(Expression::FunctionCall(Box::new(callable), arguments));
    };

    loop {
        arguments.push(expression(parse_context)?);

        match parse_context.tokens().next() {
            Some(Token::FixedToken(FixedToken::Comma)) => (),
            Some(Token::FixedToken(FixedToken::RightParenthesis)) => break,
            _ => {
                parse_context.record_error(ParseErrorKind::UnexpectedToken);
                return Err(ShouldSynchronize::Yes);
            }
        }
    }

    Ok(Expression::FunctionCall(Box::new(callable), arguments))
}

fn primary<T: Iterator<Item = Token>>(
    parse_context: &mut ParseContext<ParseErrorKind, T>,
) -> ParseResult<Expression> {
    match parse_context
        .tokens()
        .next()
        .expect("Expected primary expression")
    {
        Token::FixedToken(FixedToken::LeftParenthesis) => {
            let expression = expression(parse_context)?;

            match parse_context.tokens().next() {
                Some(token) => match token {
                    Token::FixedToken(FixedToken::RightParenthesis) => {
                        Ok(Expression::Grouping(Box::new(expression)))
                    }
                    _ => panic!("Expected right parenthesis, instead got {:?}", token),
                },
                None => {
                    parse_context.record_error(ParseErrorKind::UnmatchedParenthesis);
                    Err(ShouldSynchronize::Yes)
                }
            }
        }
        Token::Identifier(identifier) => Ok(Expression::Variable(identifier.name)),
        Token::NumericLiteral(literal) => Ok(Expression::Value(Value::Numeric(literal.value))),
        Token::StringLiteral(literal) => Ok(Expression::Value(Value::Str(literal.value.clone()))),
        Token::FixedToken(FixedToken::True) => Ok(Expression::Value(Value::Boolean(true))),
        Token::FixedToken(FixedToken::False) => Ok(Expression::Value(Value::Boolean(false))),
        Token::FixedToken(FixedToken::Nil) => Ok(Expression::Value(Value::Nil)),
        _ => {
            parse_context.record_error(ParseErrorKind::ExpectedPrimaryExpression);
            Err(ShouldSynchronize::Yes)
        }
    }
}
