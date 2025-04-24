use lib::*;

use std::env;
use std::io;

#[derive(Debug)]
enum Error {
    ParseError,
}

fn get_ast(code: &str) -> Result<ast::Ast, Error> {
    let tokens = lexer::tokenize(&code).unwrap();
    println!("{:#?}", tokens);
    let ast = match ast::Ast::parse(tokens.into_iter()) {
        Ok(ast) => ast,
        Err(errors) => {
            for context in lexer::error_context::get_error_contexts(&code, &errors) {
                println!("{:?}, {}", context.kind(), context.column());

                println!("{}", context.line());
                println!("{}^", String::from(" ").repeat(context.column()));
            }

            return Err(Error::ParseError);
        }
    };

    for statement in ast.statements.iter() {
        println!("{}", statement);
    }

    Ok(ast)
}

fn main() -> Result<(), Error> {
    let mut args = env::args();
    let _ = args.next();

    if let Some(filename) = args.next() {
        println!("{filename}");
        interpret_file(&filename)?;
    } else {
        interpret_repl();
    }

    Ok(())
}

fn interpret_file(filename: &str) -> Result<(), Error> {
    let code = std::fs::read_to_string(filename).unwrap();
    println!("{}", code);

    let ast = get_ast(&code)?;
    let mut evaluator = evaluator::Evaluator::new();

    if let Err(error) = evaluator.evaluate(&ast) {
        println!("error: {:?}", error);
    }

    println!();
    println!("{:?}", evaluator.environment);

    Ok(())
}

fn interpret_repl() -> () {
    let mut evaluator = evaluator::Evaluator::new();

    let stdin = io::stdin();
    let mut code = String::new();

    loop {
        code.clear();

        let ast = match stdin.read_line(&mut code) {
            Ok(n) if n == 0 => return,
            Err(error) => {
                println!("Error reading input: {error}");
                continue;
            }
            _ => match get_ast(&code) {
                Ok(ast) => ast,
                Err(err) => {
                    println!("{:?}", err);
                    continue;
                }
            },
        };

        if let Err(error) = evaluator.evaluate(&ast) {
            println!("error: {:?}", error);
        }

        println!();
        println!("{:?}", evaluator.environment);
    }
}
