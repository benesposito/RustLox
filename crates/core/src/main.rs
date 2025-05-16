use lib::*;

use clap::Parser;

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

#[derive(Parser)]
#[command()]
struct Args {
    #[arg(long, default_value_t = false, help = "Print the tokens for each statement")]
    show_tokens: bool,

    #[arg(long, default_value_t = false, help = "Print the ast for each statement")]
    show_ast: bool,

    #[arg(help = "Script to execute. If not specified, enter interactive mode.")]
    script: Option<String>,
}

fn main() -> Result<(), Error> {
    let args = Args::parse();

    if let Some(filename) = args.script {
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
