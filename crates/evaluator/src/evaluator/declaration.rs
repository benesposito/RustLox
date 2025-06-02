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
            Declaration::VariableDeclaration(variable_declaration) => {
                variable_declaration.evaluate(environment)
            }
        }
    }
}

impl Evaluate for VariableDeclaration {
    fn evaluate(&self, environment: &mut Environment) -> Result<(), RuntimeError> {
        let value = match &self.value {
            Some(value) => Some(value.evaluate(environment)?),
            None => None,
        };

        environment.declare_variable(&self.identifier, Value::from(value))?;
        Ok(())
    }
}
