use clap::Parser;

use std::io;

#[derive(Debug)]
enum Error {
    ParseError,
    EvaluateError,
}

#[derive(Parser)]
#[command()]
struct Args {
    #[arg(
        long,
        default_value_t = false,
        help = "Print the tokens for each statement"
    )]
    show_tokens: bool,

    #[arg(
        long,
        default_value_t = false,
        help = "Print the ast for each statement"
    )]
    show_ast: bool,

    #[arg(help = "Script to execute. If not specified, enter interactive mode.")]
    script: Option<std::path::PathBuf>,
}

fn main() -> std::process::ExitCode {
    match run(Args::parse()) {
        Ok(()) => std::process::ExitCode::SUCCESS,
        Err(error) => std::process::ExitCode::FAILURE,
    }
}

fn run(args: Args) -> Result<(), Error> {
    Interpreter::new(args).interpret()
}

enum Mode {
    Script { path: std::path::PathBuf },
    Repl,
}

struct Interpreter {
    mode: Mode,
    show_tokens: bool,
    show_ast: bool,
}

impl Interpreter {
    pub fn new(args: Args) -> Self {
        let mode = match args.script {
            Some(path) => Mode::Script { path },
            None => Mode::Repl,
        };

        Self {
            mode,
            show_tokens: args.show_tokens,
            show_ast: args.show_ast,
        }
    }

    pub fn interpret(&self) -> Result<(), Error> {
        match &self.mode {
            Mode::Script { path } => self.interpret_file(&path),
            Mode::Repl => Ok(self.interpret_repl()),
        }
    }

    fn interpret_file(&self, path: &std::path::Path) -> Result<(), Error> {
        let code = std::fs::read_to_string(path).unwrap();
        let ast = self.lex_and_parse(&code).ok_or_else(|| Error::ParseError)?;
        let mut evaluator = evaluator::Evaluator::new();

        if let Err(error) = evaluator.evaluate(&ast) {
            println!("error: {:?}", error);
        }

        println!();
        println!("{:?}", evaluator.environment);

        Ok(())
    }

    fn interpret_repl(&self) {
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
                _ => match self.lex_and_parse(&code) {
                    Some(ast) => ast,
                    None => continue,
                },
            };

            if let Err(error) = evaluator.evaluate(&ast) {
                println!("error: {:?}", error);
            }

            println!();
            println!("{:?}", evaluator.environment);
        }
    }

    fn lex_and_parse(&self, code: &str) -> Option<parser::Ast> {
        let tokens = match lexer::tokenize(&code) {
            Ok(tokens) => tokens,
            Err(tokens) => tokens,
        };

        if self.show_tokens {
            println!("{:#?}", tokens);
        }

        let ast = match parser::Ast::new(tokens.into_iter()) {
            Ok(ast) => ast,
            Err(errors) => {
                for context in errors.error_contexts(&code) {
                    println!("{context}");
                }

                return None;
            }
        };

        if self.show_ast {
            println!("{}", ast);
        }

        Some(ast)
    }
}
