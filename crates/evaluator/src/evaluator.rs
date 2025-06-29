mod declaration;
mod expression;
mod statement;

use parser::Ast;

use crate::environment::Environment;

#[derive(Debug)]
pub enum RuntimeError {
    VariableRedefinition,
    VariableDoesNotExist,
    NotCallable,
    WrongNumberOfArguments,
    TypeError,
}

pub type EvaluatorResult<T> = Result<T, RuntimeError>;

#[derive(Debug, Clone)]
pub enum Value {
    Numeric(f64),
    String_(String),
    Boolean(bool),
    Callable(Callable),
    Nil,
}

impl Value {
    fn is_truthy(&self) -> bool {
        match self {
            Value::Boolean(bool) if !bool => false,
            Value::Nil => false,
            _ => true,
        }
    }
}

impl From<Option<Self>> for Value {
    fn from(value: Option<Self>) -> Self {
        value.unwrap_or(Value::Nil)
    }
}

impl std::fmt::Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Numeric(value) => write!(f, "{value}"),
            Value::String_(value) => write!(f, "{value}"),
            Value::Boolean(value) => write!(f, "{value}"),
            Value::Callable(_) => write!(f, "<callable>"),
            Value::Nil => write!(f, "nil"),
        }
    }
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

trait EvaluateValue {
    fn evaluate(&self, environment: &mut Environment) -> Result<Value, RuntimeError>;
}

trait Evaluate {
    fn evaluate(&self, environment: &mut Environment) -> Result<(), RuntimeError>;
}

pub struct Evaluator {
    pub environment: Environment,
}

impl Evaluator {
    pub fn new() -> Self {
        Evaluator {
            environment: Environment::new(),
        }
    }

    pub fn evaluate(&mut self, ast: &Ast) -> Result<(), RuntimeError> {
        for declaration in &ast.program.declarations {
            declaration.evaluate(&mut self.environment)?;
        }

        Ok(())
    }
}
