use parser::grammar::{Binary, BinaryOperator, Expression, Primary, Unary, UnaryOperator};

use crate::environment::Environment;
use crate::evaluator::*;

impl EvaluateValue for Expression {
    fn evaluate(&self, environment: &mut Environment) -> EvaluatorResult<Value> {
        match self {
            Expression::Assignment { identifier, value } => {
                let value = value.evaluate(environment)?;

                environment.assign_variable(identifier, value)?;
                Ok(environment.lookup_variable(identifier).unwrap())
            }
            Expression::Unary(unary) => unary.evaluate(environment),
            Expression::Binary(binary) => binary.evaluate(environment),
            Expression::Primary(primary) => primary.evaluate(environment),
        }
    }
}

impl EvaluateValue for Unary {
    fn evaluate(&self, environment: &mut Environment) -> EvaluatorResult<Value> {
        match self.operator {
            UnaryOperator::Negate => match self.right.evaluate(environment)? {
                Value::Numeric(value) => Ok(Value::Numeric(-value)),
                _ => todo!("- not yet supported for types"),
            },
            UnaryOperator::Not => match self.right.evaluate(environment)? {
                Value::Boolean(value) => Ok(Value::Boolean(!value)),
                _ => todo!("! operator not yet supported for types"),
            },
        }
    }
}

impl EvaluateValue for Binary {
    fn evaluate(&self, environment: &mut Environment) -> EvaluatorResult<Value> {
        match self.operator {
            BinaryOperator::Multiplication => {
                match (
                    self.left.evaluate(environment)?,
                    self.right.evaluate(environment)?,
                ) {
                    (Value::Numeric(left_value), Value::Numeric(right_value)) => {
                        Ok(Value::Numeric(left_value * right_value))
                    }
                    _ => todo!("* operator not supported for types"),
                }
            }
            BinaryOperator::Division => {
                match (
                    self.left.evaluate(environment)?,
                    self.right.evaluate(environment)?,
                ) {
                    (Value::Numeric(left_value), Value::Numeric(right_value)) => {
                        Ok(Value::Numeric(left_value / right_value))
                    }
                    _ => todo!("/ operator not supported for types"),
                }
            }
            BinaryOperator::Addition => {
                match (
                    self.left.evaluate(environment)?,
                    self.right.evaluate(environment)?,
                ) {
                    (Value::Numeric(left_value), Value::Numeric(right_value)) => {
                        Ok(Value::Numeric(left_value + right_value))
                    }
                    (Value::String_(left_value), Value::String_(right_value)) => {
                        Ok(Value::String_(left_value + &right_value))
                    }
                    _ => todo!("+ operator not supported for types"),
                }
            }
            BinaryOperator::Subtraction => {
                match (
                    self.left.evaluate(environment)?,
                    self.right.evaluate(environment)?,
                ) {
                    (Value::Numeric(left_value), Value::Numeric(right_value)) => {
                        Ok(Value::Numeric(left_value - right_value))
                    }
                    _ => todo!("- operator not supported for types"),
                }
            }
            BinaryOperator::Equality => {
                match (
                    self.left.evaluate(environment)?,
                    self.right.evaluate(environment)?,
                ) {
                    (Value::Numeric(left_value), Value::Numeric(right_value)) => {
                        Ok(Value::Boolean(left_value == right_value))
                    }
                    (Value::Boolean(left_value), Value::Boolean(right_value)) => {
                        Ok(Value::Boolean(left_value == right_value))
                    }
                    _ => todo!("== operator not supported for types"),
                }
            }
            BinaryOperator::And => {
                let lhs = self.left.evaluate(environment)?;

                match lhs {
                    Value::Boolean(lhs_bool) if !lhs_bool => Ok(lhs),
                    Value::Boolean(lhs_bool) if lhs_bool => {
                        let rhs = self.right.evaluate(environment)?;

                        match rhs {
                            Value::Boolean(_) => Ok(rhs),
                            _ => todo!("or operator not supported for types"),
                        }
                    }
                    _ => todo!("or operator not supported for types"),
                }
            }
            BinaryOperator::Or => {
                let lhs = self.left.evaluate(environment)?;

                match lhs {
                    Value::Boolean(lhs_bool) if lhs_bool => Ok(lhs),
                    Value::Boolean(lhs_bool) if !lhs_bool => {
                        let rhs = self.right.evaluate(environment)?;

                        match rhs {
                            Value::Boolean(_) => Ok(rhs),
                            _ => todo!("or operator not supported for types"),
                        }
                    }
                    _ => todo!("or operator not supported for types"),
                }
            }
            BinaryOperator::Inequality => {
                match (
                    self.left.evaluate(environment)?,
                    self.right.evaluate(environment)?,
                ) {
                    (Value::Numeric(left_value), Value::Numeric(right_value)) => {
                        Ok(Value::Boolean(left_value != right_value))
                    }
                    (Value::Boolean(left_value), Value::Boolean(right_value)) => {
                        Ok(Value::Boolean(left_value != right_value))
                    }
                    _ => todo!("!= operator not supported for types"),
                }
            }
            BinaryOperator::GreaterThan => {
                match (
                    self.left.evaluate(environment)?,
                    self.right.evaluate(environment)?,
                ) {
                    (Value::Numeric(left_value), Value::Numeric(right_value)) => {
                        Ok(Value::Boolean(left_value > right_value))
                    }
                    _ => todo!("> operator not supported for types"),
                }
            }
            BinaryOperator::GreaterThanOrEqualTo => {
                match (
                    self.left.evaluate(environment)?,
                    self.right.evaluate(environment)?,
                ) {
                    (Value::Numeric(left_value), Value::Numeric(right_value)) => {
                        Ok(Value::Boolean(left_value >= right_value))
                    }
                    _ => todo!(">= operator not supported for types"),
                }
            }
            BinaryOperator::LessThan => {
                match (
                    self.left.evaluate(environment)?,
                    self.right.evaluate(environment)?,
                ) {
                    (Value::Numeric(left_value), Value::Numeric(right_value)) => {
                        Ok(Value::Boolean(left_value < right_value))
                    }
                    _ => todo!("< operator not supported for types"),
                }
            }
            BinaryOperator::LessThanOrEqualTo => {
                match (
                    self.left.evaluate(environment)?,
                    self.right.evaluate(environment)?,
                ) {
                    (Value::Numeric(left_value), Value::Numeric(right_value)) => {
                        Ok(Value::Boolean(left_value <= right_value))
                    }
                    _ => todo!("<= operator not supported for types"),
                }
            }
        }
    }
}

impl EvaluateValue for Primary {
    fn evaluate(&self, environment: &mut Environment) -> EvaluatorResult<Value> {
        match self {
            Primary::Call {
                callable,
                arguments,
            } => {
                let Value::Callable(callable) = callable.evaluate(environment)? else {
                    return Err(RuntimeError::NotCallable);
                };

                if callable.arity() != arguments.len() {
                    return Err(RuntimeError::WrongNumberOfArguments);
                }

                let arguments = arguments
                    .into_iter()
                    .map(|arg| arg.evaluate(environment))
                    .collect::<Result<_, _>>()?;

                Ok(callable.call(&arguments))
            }
            Primary::True => Ok(Value::Boolean(true)),
            Primary::False => Ok(Value::Boolean(false)),
            Primary::Nil => Ok(Value::Nil),
            Primary::Number(value) => Ok(Value::Numeric(*value)),
            Primary::String_(value) => Ok(Value::String_(value.clone())),
            Primary::Identifier(identifier) => match environment.lookup_variable(identifier) {
                Some(value) => Ok(value.clone()),
                None => Err(RuntimeError::VariableDoesNotExist),
            },
            Primary::Grouping(expression) => expression.evaluate(environment),
        }
    }
}
