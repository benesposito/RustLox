mod evaluator;
mod parser;
mod pretty_printer;

use crate::lexer::Token;

use std::iter::Peekable;

#[derive(Clone)]
pub enum Value {
    Numeric(f64),
    Str(String),
    Boolean(bool),
    Nil,
}

pub enum UnaryOperator {
    Negate,
    Not,
}

#[allow(dead_code)]
pub enum BinaryOperator {
    Equality,
    Inequality,
    GreaterThan,
    GreaterThanOrEqualTo,
    LessThan,
    LessThanOrEqualTo,
    And,
    Or,
    Addition,
    Subtraction,
    Multiplication,
    Division,
}

pub enum Expression {
    Value(Value),
    Unary(UnaryOperator, Box<Expression>),
    Binary(Box<Expression>, BinaryOperator, Box<Expression>),
    Grouping(Box<Expression>),
}

pub enum Statement {
    Expression(Expression),
    Print(Expression),
}

#[derive(Debug)]
#[allow(dead_code)]
pub enum ParseErrorKind {
    UnmatchedParenthesis,
    ExpectedPrimaryExpressionBefore(Token),
    ExpectedEndOfExpression,
    ExpectedSemicolon,
}

#[derive(Debug)]
#[allow(dead_code)]
pub struct ParseError {
    kind: ParseErrorKind,
    line_index: usize,
    token_index: usize,
}

type IntermediateResult<T> = Result<T, ParseErrorKind>;

impl Expression {
    pub fn parse(tokens: &mut Peekable<impl Iterator<Item = Token>>) -> IntermediateResult<Self> {
        parser::equality(tokens)
    }

    pub fn evaluate(&self) -> Value {
        evaluator::evaluate_expression(self)
    }
}

impl Statement {
    pub fn parse(tokens: &mut impl ExactSizeIterator<Item = Token>) -> Result<Self, ParseError> {
        let orig_len = tokens.len();
        let mut tokens = tokens.peekable();

        match parser::statement(&mut tokens) {
            Ok(statement) => Ok(statement),
            error => error,
        }
        .map_err(|kind| ParseError {
            kind,
            line_index: 0,
            token_index: orig_len - tokens.len(),
        })
    }

    pub fn evaluate(&self) -> Value {
        evaluator::evaluate_statement(self)
    }
}

use std::fmt;

impl std::fmt::Display for Expression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Expression::Value(value) => write!(f, "{}", value),
            Expression::Unary(operator, right) => {
                write!(f, "({} {})", operator, right)
            }
            Expression::Binary(left, operator, right) => {
                write!(f, "({} {} {})", operator, left, right)
            }
            Expression::Grouping(expression) => write!(f, "(group {})", expression),
        }
    }
}

impl std::fmt::Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Value::Numeric(value) => write!(f, "{}", value),
            Value::Str(value) => write!(f, "\"{}\"", value),
            Value::Boolean(value) => write!(f, "{}", value),
            Value::Nil => write!(f, "nil"),
        }
    }
}

impl std::fmt::Display for UnaryOperator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> fmt::Result {
        match self {
            UnaryOperator::Negate => write!(f, "-"),
            UnaryOperator::Not => write!(f, "!"),
        }
    }
}

impl std::fmt::Display for BinaryOperator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> fmt::Result {
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
