use crate::ast::Ast;
use crate::environment::Environment;

#[derive(Debug)]
pub enum RuntimeError {
    VariableDoesNotExist,
    NotCallable,
    WrongNumberOfArguments,
}

pub struct Evaluator<'a> {
    pub environment: Environment<'a>,
}

impl<'a> Evaluator<'_> {
    pub fn new() -> Self {
        Evaluator {
            environment: Environment::global(),
        }
    }

    pub fn evaluate(&mut self, ast: &Ast) -> Result<(), RuntimeError> {
        for statement in ast.statements.iter() {
            statement.evaluate(&mut self.environment)?;
        }

        Ok(())
    }
}
