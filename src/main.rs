mod ast;
mod lexer;

const filename: &str = "expression.lox";

fn main() {
    let contents = std::fs::read_to_string(filename).unwrap();
    let tokens = lexer::tokenize(&contents).unwrap();

    println!("{}", contents);
    //println!("{:#?}", tokens);

    match ast::Ast::parse(tokens.into_iter()) {
        Ok(ast) => {
            for statement in ast.statements {
                if let Some(result) = statement.evaluate() {
                    println!("> {}", result);
                }
            }
        }
        Err(errors) => {
            let contents = std::fs::read_to_string(filename).unwrap();

            for context in lexer::get_error_contexts(&contents, &errors) {
                println!("{:?}, {}", context.kind, context.column);

                println!("{}", context.line);
                println!("{}^", String::from(" ").repeat(context.column - 1));
            }
        }
    }
}
