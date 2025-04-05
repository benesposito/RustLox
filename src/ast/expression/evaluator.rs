use super::{BinaryOperator, Expression, UnaryOperator, Value};
use crate::environment::Environment;

pub fn evaluate(expression: &Expression, environment: &mut Environment) -> Value {
    match expression {
        Expression::Grouping(expression) => expression.evaluate(environment),
        Expression::Value(value) => value.clone(),
        Expression::Unary(operator, expression) => match operator {
            UnaryOperator::Negate => match expression.evaluate(environment) {
                Value::Numeric(value) => Value::Numeric(-value),
                _ => todo!("- not yet supported for types"),
            },
            UnaryOperator::Not => match expression.evaluate(environment) {
                Value::Boolean(value) => Value::Boolean(!value),
                _ => todo!("! operator not yet supported for types"),
            },
        },
        Expression::Binary(left_expression, operator, right_expression) => match operator {
            BinaryOperator::Multiplication => {
                match (
                    left_expression.evaluate(environment),
                    right_expression.evaluate(environment),
                ) {
                    (Value::Numeric(left_value), Value::Numeric(right_value)) => {
                        Value::Numeric(left_value * right_value)
                    }
                    _ => todo!("* operator not supported for types"),
                }
            }
            BinaryOperator::Division => {
                match (
                    left_expression.evaluate(environment),
                    right_expression.evaluate(environment),
                ) {
                    (Value::Numeric(left_value), Value::Numeric(right_value)) => {
                        Value::Numeric(left_value / right_value)
                    }
                    _ => todo!("/ operator not supported for types"),
                }
            }
            BinaryOperator::Addition => {
                match (
                    left_expression.evaluate(environment),
                    right_expression.evaluate(environment),
                ) {
                    (Value::Numeric(left_value), Value::Numeric(right_value)) => {
                        Value::Numeric(left_value + right_value)
                    }
                    (Value::Str(left_value), Value::Str(right_value)) => {
                        Value::Str(left_value + &right_value)
                    }
                    _ => todo!("+ operator not supported for types"),
                }
            }
            BinaryOperator::Subtraction => {
                match (
                    left_expression.evaluate(environment),
                    right_expression.evaluate(environment),
                ) {
                    (Value::Numeric(left_value), Value::Numeric(right_value)) => {
                        Value::Numeric(left_value - right_value)
                    }
                    _ => todo!("- operator not supported for types"),
                }
            }
            BinaryOperator::Equality => {
                match (
                    left_expression.evaluate(environment),
                    right_expression.evaluate(environment),
                ) {
                    (Value::Numeric(left_value), Value::Numeric(right_value)) => {
                        Value::Boolean(left_value == right_value)
                    }
                    _ => todo!("== operator not supported for types"),
                }
            }
            BinaryOperator::Inequality => {
                match (
                    left_expression.evaluate(environment),
                    right_expression.evaluate(environment),
                ) {
                    (Value::Numeric(left_value), Value::Numeric(right_value)) => {
                        Value::Boolean(left_value != right_value)
                    }
                    _ => todo!("!= operator not supported for types"),
                }
            }
            _ => todo!("Binary operator not yet supported"),
        },
    }
}
