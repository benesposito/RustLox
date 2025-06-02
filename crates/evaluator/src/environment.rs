use crate::evaluator::{Callable, EvaluatorResult, RuntimeError, Value};

use std::collections::HashMap;
use std::time::SystemTime;

#[derive(Debug)]
pub struct Environment {
    stack: Vec<Frame>,
}

#[derive(Debug)]
struct Frame {
    variables: HashMap<String, Value>,
}

impl Environment {
    pub fn new() -> Self {
        let mut environment = Self { stack: Vec::new() };
        environment.push(); /* global frame */

        environment.declare_variable(
            "time",
            Value::Callable(Callable::new(0, |_| {
                Value::Numeric(
                    SystemTime::now()
                        .duration_since(SystemTime::UNIX_EPOCH)
                        .unwrap()
                        .as_secs() as f64,
                )
            })),
        ).unwrap();

        environment
    }

    pub fn push(&mut self) {
        self.stack.push(Frame::new())
    }

    pub fn pop(&mut self) {
        self.stack.pop();
    }

    pub fn declare_variable(&mut self, identifier: &str, value: Value) -> EvaluatorResult<()> {
        let len = self.stack.len();

        /* this is safe because the vector will never be empty */
        unsafe {
            self.stack
                .get_unchecked_mut(len - 1)
                .declare_variable(identifier, value)
        }
    }

    pub fn lookup_variable(&self, identifier: &str) -> Option<Value> {
        for frame in self.stack.iter().rev() {
            if let Some(value) = frame.lookup_variable(identifier) {
                return Some(value);
            }
        }

        None
    }

}

impl Frame {
    pub fn new() -> Self {
        Self {
            variables: HashMap::new(),
        }
    }

    pub fn declare_variable(&mut self, identifier: &str, value: Value) -> EvaluatorResult<()> {
        if self.variables.contains_key(identifier) {
            return Err(RuntimeError::VariableRedefinition);
        }

        self.variables.insert(String::from(identifier), value);
        Ok(())
    }

    pub fn lookup_variable(&self, identifier: &str) -> Option<Value> {
        self.variables.get(identifier).map(|value| value.clone())
    }

    pub fn lookup_variable_mut(&mut self, identifier: &str) -> Option<&mut Value> {
        self.variables.get_mut(identifier)
    }
}
