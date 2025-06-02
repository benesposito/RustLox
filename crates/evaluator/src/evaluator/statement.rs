use crate::evaluator::*;
use parser::grammar::*;

impl Evaluate for Statement {
    fn evaluate(&self, environment: &mut Environment) -> Result<(), RuntimeError> {
        match self {
            Statement::ExpressionStatement(expression) => expression.evaluate(environment).map(|_| ()),
            Statement::Block(block) => {
                environment.push();

                for statement in &block.statements {
                    statement.evaluate(environment)?;
                }

                environment.pop();

                Ok(())
            }
            Statement::IfStatement {
                condition,
                then,
                else_,
            } => {
                let Value::Boolean(condition) = condition.evaluate(environment)? else {
                    return Err(RuntimeError::TypeError);
                };

                if condition {
                    then.evaluate(environment)
                } else {
                    else_.as_ref().map_or(Ok(()), |e| e.evaluate(environment))
                }
            }
            Statement::WhileStatement { condition, body } => {
                while condition.evaluate(environment)?.is_truthy() {
                    body.evaluate(environment)?;
                }
                Ok(())
            }
            Statement::PrintStatement(expression) => {
                println!("{}", expression.evaluate(environment)?);
                Ok(())
            }
        }
    }
}
