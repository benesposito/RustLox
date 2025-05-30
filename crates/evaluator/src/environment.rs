use crate::evaluator::{Callable, Value};

use std::collections::HashMap;
use std::time::SystemTime;

#[derive(Debug)]
pub struct Environment<'a> {
    parent: Option<&'a Environment<'a>>,
    variables: HashMap<String, Value>,
}

impl<'a> Environment<'_> {
    pub fn global() -> Environment<'a> {
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

        Environment {
            parent: None,
            variables,
        }
    }

    pub fn inner(parent: &'a Environment) -> Environment<'a> {
        Environment {
            parent: Some(parent),
            variables: HashMap::new(),
        }
    }

    pub fn declare_variable(&mut self, identifier: &str) {
        self.variables.insert(String::from(identifier), Value::Nil);
    }

    pub fn define_variable(&mut self, identifier: &str, value: Value) {
        self.variables.insert(String::from(identifier), value);
    }

    pub fn lookup_variable(&self, identifier: &str) -> Option<&Value> {
        match self.variables.get(identifier) {
            Some(value) => Some(&value),
            None => self.parent.and_then(|env| env.lookup_variable(identifier)),
        }
    }
}
