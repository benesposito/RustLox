use super::*;

pub fn evaluate_expression(expression: &Expression) -> Value {
    match expression {
        Expression::Value(value) => value.clone(),
        Expression::Unary(operator, expression) => match operator {
            UnaryOperator::Negate => match expression.evaluate() {
                Value::Numeric(value) => Value::Numeric(-value),
                _ => todo!("- not yet supported for types"),
            },
            UnaryOperator::Not => match expression.evaluate() {
                Value::Boolean(value) => Value::Boolean(!value),
                _ => todo!("! operator not yet supported for types"),
            },
        },
        Expression::Binary(left_expression, operator, right_expression) => match operator {
            BinaryOperator::Multiplication => {
                match (left_expression.evaluate(), right_expression.evaluate()) {
                    (Value::Numeric(left_value), Value::Numeric(right_value)) => {
                        Value::Numeric(left_value * right_value)
                    }
                    _ => todo!("* operator not supported for types"),
                }
            }
            BinaryOperator::Division => {
                match (left_expression.evaluate(), right_expression.evaluate()) {
                    (Value::Numeric(left_value), Value::Numeric(right_value)) => {
                        Value::Numeric(left_value / right_value)
                    }
                    _ => todo!("/ operator not supported for types"),
                }
            }
            BinaryOperator::Addition => {
                match (left_expression.evaluate(), right_expression.evaluate()) {
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
                match (left_expression.evaluate(), right_expression.evaluate()) {
                    (Value::Numeric(left_value), Value::Numeric(right_value)) => {
                        Value::Numeric(left_value - right_value)
                    }
                    _ => todo!("- operator not supported for types"),
                }
            }
            BinaryOperator::Equality => {
                match (left_expression.evaluate(), right_expression.evaluate()) {
                    (Value::Numeric(left_value), Value::Numeric(right_value)) => {
                        Value::Boolean(left_value == right_value)
                    }
                    _ => todo!("== operator not supported for types"),
                }
            }
            BinaryOperator::Inequality => {
                match (left_expression.evaluate(), right_expression.evaluate()) {
                    (Value::Numeric(left_value), Value::Numeric(right_value)) => {
                        Value::Boolean(left_value != right_value)
                    }
                    _ => todo!("!= operator not supported for types"),
                }
            }
            _ => todo!("Binary operator not yet supported"),
        },
        Expression::Grouping(expression) => expression.evaluate(),
    }
}

pub fn evaluate_statement(statement: &Statement) -> Value {
    match statement {
        Statement::Expression(expression) => expression.evaluate(),
        Statement::Print(expression) => expression.evaluate(),
    }
}
