use crate::evaluator::*;
use parser::grammar::*;

impl Evaluate for Program {
    fn evaluate(&self, environment: &mut Environment) -> Result<(), RuntimeError> {
        todo!()
    }
}

impl Evaluate for Declaration {
    fn evaluate(&self, environment: &mut Environment) -> Result<(), RuntimeError> {
        match self {
            Declaration::Statement(statement) => statement.evaluate(environment),
            Declaration::VariableDeclaration(identifier, None) => {
                environment.declare_variable(identifier);
                Ok(())
            }
            Declaration::VariableDeclaration(identifier, Some(expression)) => {
                let value = expression.evaluate(environment)?;
                environment.define_variable(identifier, value);
                Ok(())
            }
        }
    }
}
