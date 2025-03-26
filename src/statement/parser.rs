use super::*;

use crate::lexer::Token;

pub fn statement(
    tokens: &mut Peekable<impl Iterator<Item = Token>>,
) -> IntermediateResult<Statement> {
    let statement = match tokens.peek().unwrap() {
        Token::Print => {
            tokens.next();
            Statement::Print(Expression::parse(tokens)?)
        }
        _ => Statement::Expression(Expression::parse(tokens)?),
    };

    match tokens.next() {
        Some(token) => match token {
            Token::Semicolon => Ok(statement),
            _ => {
                while let Some(token) = tokens.next() {
                    match token {
                        Token::Semicolon => {
                            break;
                        }
                        _ => (),
                    }
                }
                Err(ParseErrorKind::ExpectedSemicolon)
            }
        },
        None => Err(ParseErrorKind::ExpectedSemicolon),
    }
}

pub fn equality(
    tokens: &mut Peekable<impl Iterator<Item = Token>>,
) -> IntermediateResult<Expression> {
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

pub fn comparison(
    tokens: &mut Peekable<impl Iterator<Item = Token>>,
) -> IntermediateResult<Expression> {
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

pub fn term(tokens: &mut Peekable<impl Iterator<Item = Token>>) -> IntermediateResult<Expression> {
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

pub fn factor(
    tokens: &mut Peekable<impl Iterator<Item = Token>>,
) -> IntermediateResult<Expression> {
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

pub fn unary(tokens: &mut Peekable<impl Iterator<Item = Token>>) -> IntermediateResult<Expression> {
    let operator = match tokens.peek().expect("Expected unary or primary expression") {
        Token::Minus => UnaryOperator::Negate,
        Token::Bang => UnaryOperator::Not,
        _ => return primary(tokens),
    };

    tokens.next();
    Ok(Expression::Unary(operator, Box::new(unary(tokens)?)))
}

pub fn primary(
    tokens: &mut Peekable<impl Iterator<Item = Token>>,
) -> IntermediateResult<Expression> {
    match tokens.next().expect("Expected primary expression") {
        Token::NumericLiteral(value) => Ok(Expression::Value(Value::Numeric(value))),
        Token::StringLiteral(value) => Ok(Expression::Value(Value::Str(value.clone()))),
        Token::True => Ok(Expression::Value(Value::Boolean(true))),
        Token::False => Ok(Expression::Value(Value::Boolean(false))),
        Token::Nil => Ok(Expression::Value(Value::Nil)),
        Token::LeftParenthesis => {
            let expression = Expression::parse(tokens)?;

            match tokens.next() {
                Some(token) => match token {
                    Token::RightParenthesis => Ok(Expression::Grouping(Box::new(expression))),
                    _ => panic!("Expected right parenthesis, instead got {:?}", token),
                },
                None => Err(ParseErrorKind::UnmatchedParenthesis),
            }
        }
        token => Err(ParseErrorKind::ExpectedPrimaryExpressionBefore(token)),
    }
}

impl ParseError {
    pub fn token_index(self: &Self) -> usize {
        self.token_index
    }
}
