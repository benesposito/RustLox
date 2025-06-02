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
            Declaration::VariableDeclaration(identifier, value) => {
                let value = match value {
                    Some(value) => Some(value.evaluate(environment)?),
                    None => None,
                };
        
                environment.declare_variable(identifier, Value::from(value))?;
                Ok(())
            }
        }
    }
}
