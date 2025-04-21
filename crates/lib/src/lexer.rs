mod fixed_token;
mod identifier;
mod numeric_literal;
mod string_literal;

pub use fixed_token::FixedToken;
pub use identifier::Identifier;
pub use numeric_literal::NumericLiteral;
pub use string_literal::StringLiteral;

trait LexImpl {
    fn extract_impl(input: &mut &str) -> Option<Self>
    where
        Self: Sized;
}

#[allow(private_bounds)]
pub trait Lex: LexImpl {
    /// Consume and return a token from a string slice.
    ///
    /// # Examples
    /// ```
    /// # use lib::lexer::*;
    /// let mut code = "var x = 5;";
    ///
    /// let token = FixedToken::extract(&mut code).unwrap();
    /// assert!(matches!(token, FixedToken::Var));
    ///
    /// let token = Identifier::extract(&mut code).unwrap();
    /// assert_eq!(token, Identifier{name: String::from("x")});
    ///
    /// let token = FixedToken::extract(&mut code).unwrap();
    /// assert!(matches!(token, FixedToken::Equal));
    ///
    /// let token = NumericLiteral::extract(&mut code).unwrap();
    /// assert_eq!(token, NumericLiteral{value: 5f64});
    ///
    /// let token = FixedToken::extract(&mut code).unwrap();
    /// assert!(matches!(token, FixedToken::Semicolon));
    /// ```
    fn extract(input: &mut &str) -> Option<Self>
    where
        Self: Sized,
    {
        let token = Self::extract_impl(input);
        *input = input.trim_start_matches(is_skippable_whitespace);
        token
    }
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
        } else if {
            let c = input
                .chars()
                .next()
                .expect("Expression is unexpectedly empty");
            c.is_ascii_digit() || c == '-' || c == '+'
        } {
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
        for _ in 0..(error.token_index - 1) {
            while is_skippable_whitespace(input.chars().next().unwrap()) {
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

    if !partials.is_empty() {
        loop {
            let original_input_len = input.len();
            match Token::extract(&mut input).expect("Tokens have already been validated") {
                Token::FixedToken(FixedToken::Newline) => break,
                _ => column += original_input_len - input.len(),
            }
        }

        contexts.extend(partials.into_iter().map(|partial| ErrorContext {
            kind: partial.kind.clone(),
            line: &start_of_line[0..column],
            column: partial.column,
        }));
    }

    contexts
}

fn is_skippable_whitespace(c: char) -> bool {
    c.is_whitespace() && c != '\n'
}
