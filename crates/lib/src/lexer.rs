mod fixed_token;
mod identifier;
mod numeric_literal;
mod string_literal;

pub use fixed_token::FixedToken;
pub use identifier::Identifier;
pub use numeric_literal::NumericLiteral;
pub use string_literal::StringLiteral;

#[derive(Debug)]
enum LexError {
    NoKind,
    NumericContainsAlpha,
    UnclosedString,
}

type LexResult<T> = std::result::Result<T, LexError>;

pub trait LookaheadLex {
    fn is_kind(input: &str) -> bool;

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
    fn extract(input: &mut &str) -> LexResult<Self>
    where
        Self: Sized;
}

#[derive(Debug)]
pub enum Token {
    FixedToken(FixedToken),
    Identifier(Identifier),
    StringLiteral(StringLiteral),
    NumericLiteral(NumericLiteral),
    Error(LexError),
}

impl Token {
    fn extract(input: &mut &str) -> Self {
        *input = input.trim_start_matches(is_skippable_whitespace);

        if let Some(token) = FixedToken::extract(input) {
            Token::FixedToken(token)
        } else if NumericLiteral::is_kind(input) {
            Token::from(NumericLiteral::extract(input))
        } else if StringLiteral::is_kind(input) {
            Token::from(StringLiteral::extract(input))
        } else if Identifier::is_kind(input) {
            Token::from(Identifier::extract(input))
        } else {
            /* synchronize on space */
            let new_input = input.trim_start_matches(is_skippable_whitespace);
            let token = &input[..new_input.len()];
            *input = new_input;
            //Ok(Token::Error(LexError::NoKind, String::from(input[..new_input.len()])))
            Token::Error(LexError::NoKind)
        }
    }
}

impl<T: LookaheadLex> From<LexResult<T>> for Token
where
    Token: From<T>,
{
    fn from(maybe_value: LexResult<T>) -> Self {
        match maybe_value {
            Ok(value) => Token::from(value),
            Err(e) => Token::Error(e),
        }
    }
}

pub fn tokenize(mut input: &str) -> Result<Vec<Token>, Vec<Token>> {
    let mut tokens: Vec<Token> = Vec::new();
    let mut has_error = false;

    input = input.trim_start_matches(|c: char| c.is_whitespace() && c != '\n');

    while !input.is_empty() {
        let token = Token::extract(&mut input);
        has_error &= matches!(token, Token::Error(_));
        tokens.push(token);
    }

    match has_error {
        false => Ok(tokens),
        true => Err(tokens),
    }
}

pub mod error_context {
    use super::*;

    pub struct RecordedError<ErrorKind> {
        pub kind: ErrorKind,
        pub token_index: usize,
    }

    pub struct ErrorRecorder<ErrorKind> {
        //tokens: &'a TokenIterator,
        original_length: usize,
        pub errors: Vec<RecordedError<ErrorKind>>,
    }

    impl<ErrorKind: Clone> ErrorRecorder<ErrorKind> {
        pub fn new(tokens: &impl ExactSizeIterator) -> Self {
            /* take in iterator ref, come up with new name */
            Self {
                //tokens,
                original_length: tokens.len(),
                errors: Vec::new(),
            }
        }

        pub fn record(&mut self, tokens: &impl ExactSizeIterator, kind: ErrorKind) {
            self.errors.push(RecordedError {
                kind,
                token_index: self.original_length - tokens.len(),
            });
        }

        pub fn error_contexts<'a>(&self, input: &'a str) -> Vec<ErrorContext<'a, ErrorKind>> {
            get_error_contexts(input, &self.errors)
        }
    }

    #[derive(Debug)]
    struct PartialErrorContext<ErrorKind> {
        kind: ErrorKind,
        column: usize,
    }

    pub struct ErrorContext<'a, ErrorKind> {
        partial: PartialErrorContext<ErrorKind>,
        line: &'a str,
    }

    impl<'a, ErrorKind: Clone> ErrorContext<'a, ErrorKind> {
        pub fn kind(&self) -> ErrorKind {
            self.partial.kind.clone()
        }

        pub fn column(&self) -> usize {
            self.partial.column
        }

        pub fn line(&self) -> &'a str {
            self.line
        }
    }

    pub fn get_error_contexts<'a, ErrorKind: Clone>(
        mut input: &'a str,
        errors: &Vec<RecordedError<ErrorKind>>,
    ) -> Vec<ErrorContext<'a, ErrorKind>> {
        let mut contexts: Vec<ErrorContext<ErrorKind>> = Vec::with_capacity(errors.len());
        let mut partials: Vec<PartialErrorContext<ErrorKind>> = Vec::new();

        let errors = errors.iter().scan(0, |prev_token_index, error| {
            let new_error = RecordedError::<ErrorKind> {
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
                match Token::extract(&mut input) {
                    Token::FixedToken(FixedToken::Newline) => {
                        contexts.extend(partials.into_iter().map(|partial| ErrorContext {
                            partial: partial,
                            line: &start_of_line[0..column],
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
                match Token::extract(&mut input) {
                    Token::FixedToken(FixedToken::Newline) => break,
                    _ => column += original_input_len - input.len(),
                }
            }

            contexts.extend(partials.into_iter().map(|partial| ErrorContext {
                partial: partial,
                line: &start_of_line[0..column],
            }));
        }

        contexts
    }
}

fn is_skippable_whitespace(c: char) -> bool {
    c.is_whitespace() && c != '\n'
}
