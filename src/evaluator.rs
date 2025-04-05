use crate::ast::Ast;
use crate::environment::Environment;

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

    pub fn evaluate(&mut self) {
        for statement in self.ast.statements.iter() {
            if let Some(result) = statement.evaluate(&mut self.environment) {
                println!("> {}", result);
            }
        }
    }
}
