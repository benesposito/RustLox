use crate::ast::Ast;
use crate::environment::Environment;

#[derive(Debug)]
pub enum RuntimeError {
    VariableDoesNotExist,
}

pub struct Evaluator {
    ast: Ast,
    pub environment: Environment,
}

impl Evaluator {
    pub fn new(ast: Ast) -> Self {
        Evaluator {
            ast,
            environment: Environment::new(),
        }
    }

    pub fn evaluate(&mut self) -> Result<(), RuntimeError> {
        for statement in self.ast.statements.iter() {
            statement.evaluate(&mut self.environment)?;
        }

        Ok(())
    }
}
