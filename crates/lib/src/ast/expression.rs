mod evaluator;
mod parser;

use super::ParseResult;
use crate::environment::Environment;
use crate::evaluator::RuntimeError;

use lexer::Token;

#[derive(Debug)]
pub enum Expression {
    Grouping(Box<Expression>),
    Value(Value),
    Variable(String),
    Unary(UnaryOperator, Box<Expression>),
    Binary(Box<Expression>, BinaryOperator, Box<Expression>),
    FunctionCall(Box<Expression>, Vec<Expression>),
}

impl Expression {
    pub fn parse(
        tokens: &mut std::iter::Peekable<impl Iterator<Item = Token>>,
    ) -> ParseResult<Self> {
        parser::expression(tokens)
    }

    pub fn evaluate(&self, environment: &mut Environment) -> Result<Value, RuntimeError> {
        evaluator::evaluate(self, environment)
    }
}

#[derive(Debug)]
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

#[derive(Debug)]
pub enum UnaryOperator {
    Negate,
    Not,
}

#[derive(Clone, Debug)]
pub struct Callable {
    arity: usize,
    function: fn(&Vec<Value>) -> Value,
}

impl Callable {
    pub fn new(arity: usize, function: fn(&Vec<Value>) -> Value) -> Self {
        Callable { arity, function }
    }

    pub fn arity(&self) -> usize {
        self.arity
    }

    pub fn call(&self, arguments: &Vec<Value>) -> Value {
        (self.function)(arguments)
    }
}

#[derive(Clone, Debug)]
pub enum Value {
    Numeric(f64),
    Str(String),
    Boolean(bool),
    Callable(Callable),
    Nil,
}

use std::fmt;

impl fmt::Display for Expression {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Expression::Variable(name) => write!(f, "{}", name),
            Expression::Value(value) => write!(f, "{}", value),
            Expression::Unary(operator, right) => {
                write!(f, "({} {})", operator, right)
            }
            Expression::Binary(left, operator, right) => {
                write!(f, "({} {} {})", operator, left, right)
            }
            Expression::Grouping(expression) => write!(f, "(group {})", expression),
            Expression::FunctionCall(callable, arguments) => write!(
                f,
                "({}{}{})",
                callable,
                if arguments.len() > 0 { " " } else { "" },
                arguments
                    .into_iter()
                    .map(|e| format!("{}", e))
                    .collect::<Vec<_>>()
                    .join(" ")
            ),
        }
    }
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Value::Numeric(value) => write!(f, "{}", value),
            Value::Str(value) => write!(f, "\"{}\"", value),
            Value::Boolean(value) => write!(f, "{}", value),
            Value::Callable(_) => write!(f, "<callable>"),
            Value::Nil => write!(f, "nil"),
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
