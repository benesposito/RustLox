mod fixed_token;
mod identifier;
mod numeric_literal;
mod string_literal;

pub use fixed_token::FixedToken;
pub use identifier::Identifier;
pub use numeric_literal::NumericLiteral;
pub use string_literal::StringLiteral;

trait Lex {
    fn extract(input: &mut &str) -> Option<Self>
    where
        Self: Sized;
}

#[derive(Debug)]
pub enum Token {
    FixedToken(FixedToken),
    Identifier(Identifier),
    StringLiteral(StringLiteral),
    NumericLiteral(NumericLiteral),
}

impl Token {
    fn extract(input: &mut &str) -> Option<Self> {
        if let Some(token) = FixedToken::extract(input) {
            Some(Token::FixedToken(token))
        } else if input
            .chars()
            .next()
            .expect("Expression is unexpectedly empty")
            .is_ascii_digit()
        {
            Some(Token::NumericLiteral(NumericLiteral::extract(input)?))
        } else if input.starts_with("\"") {
            Some(Token::StringLiteral(StringLiteral::extract(input)?))
        } else {
            Some(Token::Identifier(Identifier::extract(input)?))
        }
    }
}

pub fn tokenize(mut input: &str) -> Option<Vec<Token>> {
    let mut tokens: Vec<Token> = Vec::new();

    input = input.trim_start_matches(|c: char| c.is_whitespace() && c != '\n');

    while !input.is_empty() {
        match Token::extract(&mut input) {
            Some(token) => {
                tokens.push(token);
            }
            None => return None,
        }

        input = input.trim_start_matches(|c: char| c.is_whitespace() && c != '\n');
    }

    Some(tokens)
}

pub struct ErrorContext<'a> {
    pub kind: crate::ast::ParseErrorKind,
    pub line: &'a str,
    pub column: usize,
}

#[derive(Debug)]
struct PartialErrorContext {
    kind: crate::ast::ParseErrorKind,
    column: usize,
}

pub fn get_error_contexts<'a>(
    mut input: &'a str,
    errors: &Vec<crate::ast::ParseError>,
) -> Vec<ErrorContext<'a>> {
    let mut contexts: Vec<ErrorContext> = Vec::with_capacity(errors.len());
    let mut partials: Vec<PartialErrorContext> = Vec::new();

    let errors = errors.iter().scan(0, |prev_token_index, error| {
        let new_error = crate::ast::ParseError {
            kind: error.kind.clone(),
            token_index: error.token_index - *prev_token_index,
        };

        *prev_token_index = error.token_index;
        Some(new_error)
    });

    let mut start_of_line = input;
    let mut column: usize = 0;

    for error in errors {
        for _ in 0..error.token_index {
            while input.chars().next().unwrap().is_whitespace()
                && input.chars().next().unwrap() != '\n'
            {
                input = &input[1..];
                column += 1;
            }

            let original_input_len = input.len();
            match Token::extract(&mut input).expect("Tokens have already been validated") {
                Token::FixedToken(FixedToken::Newline) => {
                    contexts.extend(partials.into_iter().map(|partial| ErrorContext {
                        kind: partial.kind.clone(),
                        line: &start_of_line[0..column],
                        column: partial.column,
                    }));

                    partials = Vec::new();
                    start_of_line = input;
                    column = 0;
                }
                _ => {
                    column += original_input_len - input.len();
                }
            }
        }

        partials.push(PartialErrorContext {
            kind: error.kind.clone(),
            column,
        });
    }

    contexts.extend(partials.into_iter().map(|partial| ErrorContext {
        kind: partial.kind.clone(),
        line: &start_of_line[0..column],
        column: partial.column,
    }));

    contexts
}
