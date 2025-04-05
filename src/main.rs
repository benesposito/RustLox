mod ast;
mod environment;
mod evaluator;
mod lexer;

const FILENAME: &str = "statements.lox";

fn main() {
    let contents = std::fs::read_to_string(FILENAME).unwrap();
    let tokens = lexer::tokenize(&contents).unwrap();

    println!("{}", contents);

    match ast::Ast::parse(tokens.into_iter()) {
        Ok(ast) => {
            for statement in ast.statements.iter() {
                println!("{}", statement);
            }

            let mut evaluator = evaluator::Evaluator::new(ast);
            evaluator.evaluate();

            println!();
            println!("{:?}", evaluator.environment);
        }
        Err(errors) => {
            let contents = std::fs::read_to_string(FILENAME).unwrap();

            for context in lexer::get_error_contexts(&contents, &errors) {
                println!("{:?}, {}", context.kind, context.column);

                println!("{}", context.line);
                println!("{}^", String::from(" ").repeat(context.column - 1));
            }
        }
    }
}
