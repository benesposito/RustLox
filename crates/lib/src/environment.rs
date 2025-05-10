use crate::ast::expression::{Callable, Value};

use std::collections::HashMap;
use std::time::SystemTime;

#[derive(Debug)]
pub struct Environment {
    variables: HashMap<String, Value>,
}

impl Environment {
    pub fn new() -> Self {
        let mut variables = HashMap::new();

        variables.insert(
            String::from("time"),
            Value::Callable(Callable::new(0, |_| {
                Value::Numeric(
                    SystemTime::now()
                        .duration_since(SystemTime::UNIX_EPOCH)
                        .unwrap()
                        .as_secs() as f64,
                )
            })),
        );

        Environment { variables }
    }

    pub fn declare_variable(&mut self, identifier: &str) {
        self.variables.insert(String::from(identifier), Value::Nil);
    }

    pub fn define_variable(&mut self, identifier: &str, value: Value) {
        self.variables.insert(String::from(identifier), value);
    }

    pub fn lookup_variable(&self, identifier: &str) -> Option<Value> {
        self.variables.get(identifier).cloned()
    }
}
