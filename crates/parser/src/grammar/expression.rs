use lexer::{tokens::FixedToken, Token};

use crate::grammar::*;
use crate::parser::*;

impl Expression {
    pub fn parse<T: Iterator<Item = Token>>(
        parse_context: &mut ParseContext<T>,
    ) -> ParseResult<Self> {
        expression(parse_context)
    }
}

fn expression<T: Iterator<Item = Token>>(
    parse_context: &mut ParseContext<T>,
) -> ParseResult<Expression> {
    assignment(parse_context)
}

fn assignment<T: Iterator<Item = Token>>(
    parse_context: &mut ParseContext<T>,
) -> ParseResult<Expression> {
    let expr = logical_or(parse_context)?;

    if let Expression::Primary(Primary::Identifier(identifier)) = &expr {
        match parse_context.tokens().peek().expect("Expected tokens") {
            Token::FixedToken(FixedToken::Equal) => {
                parse_context.tokens().next();
                let value = Expression::parse(parse_context)?;
                return Ok(Expression::Assignment {
                    identifier: identifier.clone(),
                    value: Box::new(value),
                });
            }
            _ => (),
        }
    }

    Ok(expr)
}

fn logical_or<T: Iterator<Item = Token>>(
    parse_context: &mut ParseContext<T>,
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

        expression = Expression::Binary(Binary {
            left: Box::new(expression),
            operator,
            right: Box::new(right),
        });
    }
}

fn logical_and<T: Iterator<Item = Token>>(
    parse_context: &mut ParseContext<T>,
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

        expression = Expression::Binary(Binary {
            left: Box::new(expression),
            operator,
            right: Box::new(right),
        });
    }
}

fn equality<T: Iterator<Item = Token>>(
    parse_context: &mut ParseContext<T>,
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

        expression = Expression::Binary(Binary {
            left: Box::new(expression),
            operator,
            right: Box::new(right),
        });
    }
}

fn comparison<T: Iterator<Item = Token>>(
    parse_context: &mut ParseContext<T>,
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
        expression = Expression::Binary(Binary {
            left: Box::new(expression),
            operator,
            right: Box::new(right),
        });
    }
}

fn term<T: Iterator<Item = Token>>(parse_context: &mut ParseContext<T>) -> ParseResult<Expression> {
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

        expression = Expression::Binary(Binary {
            left: Box::new(expression),
            operator,
            right: Box::new(right),
        });
    }
}

fn factor<T: Iterator<Item = Token>>(
    parse_context: &mut ParseContext<T>,
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

        expression = Expression::Binary(Binary {
            left: Box::new(expression),
            operator,
            right: Box::new(right),
        });
    }
}

fn unary<T: Iterator<Item = Token>>(
    parse_context: &mut ParseContext<T>,
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

    Ok(Expression::Unary(Unary {
        operator,
        right: Box::new(unary(parse_context)?),
    }))
}

fn call<T: Iterator<Item = Token>>(parse_context: &mut ParseContext<T>) -> ParseResult<Expression> {
    let callable = Primary::parse(parse_context)?;

    let Some(Token::FixedToken(FixedToken::LeftParenthesis)) = parse_context.tokens().peek() else {
        return Ok(Expression::Primary(callable));
    };

    parse_context.tokens().next();

    let mut arguments: Vec<Expression> = Vec::new();

    if let Some(Token::FixedToken(FixedToken::RightParenthesis)) = parse_context.tokens().peek() {
        parse_context.tokens().next();
        return Ok(Expression::Primary(Primary::Call {
            callable: Box::new(callable),
            arguments,
        }));
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

    Ok(Expression::Primary(Primary::Call {
        callable: Box::new(callable),
        arguments,
    }))
}

impl Primary {
    pub fn parse<T: Iterator<Item = Token>>(
        parse_context: &mut ParseContext<T>,
    ) -> ParseResult<Self> {
        match parse_context
            .tokens()
            .next()
            .expect("Expected primary expression")
        {
            Token::FixedToken(FixedToken::True) => Ok(Primary::True),
            Token::FixedToken(FixedToken::False) => Ok(Primary::False),
            Token::FixedToken(FixedToken::Nil) => Ok(Primary::Nil),
            Token::NumericLiteral(literal) => Ok(Primary::Number(literal.value)),
            Token::StringLiteral(literal) => Ok(Primary::String_(literal.value.clone())),
            Token::Identifier(identifier) => Ok(Primary::Identifier(identifier.name)),
            Token::FixedToken(FixedToken::LeftParenthesis) => {
                let expression = expression(parse_context)?;

                match parse_context.tokens().next() {
                    Some(token) => match token {
                        Token::FixedToken(FixedToken::RightParenthesis) => {
                            Ok(Primary::Grouping(Box::new(expression)))
                        }
                        _ => panic!("Expected right parenthesis, instead got {:?}", token),
                    },
                    None => {
                        parse_context.record_error(ParseErrorKind::UnmatchedParenthesis);
                        Err(ShouldSynchronize::Yes)
                    }
                }
            }
            _ => {
                parse_context.record_error(ParseErrorKind::ExpectedPrimaryExpression);
                Err(ShouldSynchronize::Yes)
            }
        }
    }
}

use std::fmt;

impl fmt::Display for Expression {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Expression::Assignment { identifier, value } => {
                write!(f, "(assign {identifier} {value})")
            }
            Expression::Primary(value) => write!(f, "{}", value),
            Expression::Unary(unary) => {
                write!(f, "({} {})", unary.operator, unary.right)
            }
            Expression::Binary(binary) => {
                write!(f, "({} {} {})", binary.operator, binary.left, binary.right)
            }
        }
    }
}

impl fmt::Display for BinaryOperator {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BinaryOperator::Equality => write!(f, "=="),
            BinaryOperator::Inequality => write!(f, "!="),
            BinaryOperator::GreaterThan => write!(f, ">"),
            BinaryOperator::GreaterThanOrEqualTo => write!(f, ">="),
            BinaryOperator::LessThan => write!(f, "<"),
            BinaryOperator::LessThanOrEqualTo => write!(f, "<="),
            BinaryOperator::And => write!(f, "and"),
            BinaryOperator::Or => write!(f, "or"),
            BinaryOperator::Addition => write!(f, "+"),
            BinaryOperator::Subtraction => write!(f, "-"),
            BinaryOperator::Multiplication => write!(f, "*"),
            BinaryOperator::Division => write!(f, "/"),
        }
    }
}

impl fmt::Display for UnaryOperator {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            UnaryOperator::Negate => write!(f, "-"),
            UnaryOperator::Not => write!(f, "!"),
        }
    }
}

impl fmt::Display for Primary {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Primary::Call {
                callable,
                arguments,
            } => write!(f, "({callable:?} {arguments:?})"),
            Primary::True => write!(f, "true"),
            Primary::False => write!(f, "false"),
            Primary::Nil => write!(f, "nil"),
            Primary::Number(value) => write!(f, "{value:?}"),
            Primary::Identifier(name) => write!(f, "{name}"),
            Primary::String_(value) => write!(f, "{value:?}"),
            Primary::Grouping(expression) => write!(f, "{expression:?}"),
        }
    }
}
