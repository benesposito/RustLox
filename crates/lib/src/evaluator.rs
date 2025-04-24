use crate::ast::Ast;
use crate::environment::Environment;

#[derive(Debug)]
pub enum RuntimeError {
    VariableDoesNotExist,
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
        for statement in ast.statements.iter() {
            statement.evaluate(&mut self.environment)?;
        }

        Ok(())
    }
}
