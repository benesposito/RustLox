use crate::evaluator::*;
use parser::grammar::*;

impl Evaluate for Statement {
    fn evaluate(&self, environment: &mut Environment) -> Result<(), RuntimeError> {
        match self {
            Statement::ExpressionStatement(expression) => expression.evaluate(environment).map(|_| ()),
            Statement::Block(block) => {
                let mut environment = Environment::inner(environment);
                for statement in &block.statements {
                    statement.evaluate(&mut environment)?
                }
                Ok(())
            }
            Statement::IfStatement { conditional, then, else_ } => {
                let Value::Boolean(conditional) = conditional.evaluate(environment)? else {
                    return Err(RuntimeError::TypeError);
                };

                if conditional {
                    then.evaluate(environment)
                } else {
                    else_.as_ref().map_or(Ok(()), |e| e.evaluate(environment))
                }
            }
            Statement::PrintStatement(expression) => {
                println!("{}", expression.evaluate(environment)?);
                Ok(())
            }
        }
    }
}
