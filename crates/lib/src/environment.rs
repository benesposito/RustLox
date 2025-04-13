use crate::ast::expression::Value;

use std::collections::HashMap;

#[derive(Debug)]
pub struct Environment {
    variables: HashMap<String, Value>,
}

impl Environment {
    pub fn new() -> Self {
        Environment {
            variables: HashMap::new(),
        }
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
