use lib::*;

const FILENAME: &str = "statements.lox";

fn main() {
    let contents = std::fs::read_to_string(FILENAME).unwrap();
    let tokens = lexer::tokenize(&contents).unwrap();

    println!("{}", contents);

    let ast = match ast::Ast::parse(tokens.into_iter()) {
        Ok(ast) => ast,
        Err(errors) => {
            let contents = std::fs::read_to_string(FILENAME).unwrap();

            for context in lexer::get_error_contexts(&contents, &errors) {
                println!("{:?}, {}", context.kind, context.column);

                println!("{}", context.line);
                println!("{}^", String::from(" ").repeat(context.column));
            }

            return;
        }
    };

    for statement in ast.statements.iter() {
        println!("{}", statement);
    }

    let mut evaluator = evaluator::Evaluator::new(ast);

    if let Err(error) = evaluator.evaluate() {
        println!("error: {:?}", error);
    }

    println!();
    println!("{:?}", evaluator.environment);
}
