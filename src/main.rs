mod lexer;
mod statement;

use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn main() {
    let f = File::open("expression.lox").unwrap();
    let reader = BufReader::new(f);

    let _: Vec<_> = reader
        .lines()
        .map(|line| {
            let line = line.unwrap();
            let tokens = lexer::tokenize(line.as_str()).unwrap();

            let statement = match statement::Statement::parse(&mut tokens.into_iter()) {
                Ok(statement) => {
                    let result = statement.evaluate();
                    println!("{}", result);
                }
                Err(error) => {
                    let column_idx =
                        lexer::get_position_by_token(line.as_str(), error.token_index()).1;

                    println!("{:?}:", error);
                    println!("{}", line);
                    println!("{}^", String::from(" ").repeat(column_idx));
                }
            };
        })
        .collect();
}
