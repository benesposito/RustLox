mod fixed_token;
mod identifier;
mod numeric_literal;
mod string_literal;

pub use fixed_token::FixedToken;
pub use identifier::Identifier;
pub use numeric_literal::NumericLiteral;
pub use string_literal::StringLiteral;

#[derive(Debug, Clone)]
pub enum LexError {
    NoTokenKind,
    NumericContainsAlpha,
    UnclosedString,
}

type LexResult<T> = std::result::Result<T, LexError>;

pub trait LookaheadLex: std::fmt::Debug {
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
            //Ok(Token::Error(LexError::NoTokenKind, String::from(input[..new_input.len()])))
            Token::Error(LexError::NoTokenKind)
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

    while !input.is_empty() {
        let token = Token::extract(&mut input);
        has_error |= matches!(token, Token::Error(_));
        tokens.push(token);
    }

    match has_error {
        false => Ok(tokens),
        true => Err(tokens),
    }
}

pub mod error_context {
    use super::*;

    #[derive(Debug)]
    pub struct RecordedError<ParseErrorKind> {
        pub kind: ParseErrorKind,
        pub token_index: usize,
    }

    pub struct ErrorRecorder<ParseErrorKind> {
        //tokens: &'a TokenIterator,
        original_length: usize,
        pub errors: Vec<RecordedError<ParseErrorKind>>,
    }

    impl<ParseErrorKind: Clone + std::fmt::Debug> ErrorRecorder<ParseErrorKind> {
        pub fn new(tokens: &impl ExactSizeIterator) -> Self {
            /* take in iterator ref, come up with new name */
            Self {
                //tokens,
                original_length: tokens.len(),
                errors: Vec::new(),
            }
        }

        pub fn record(&mut self, tokens: &impl ExactSizeIterator, kind: ParseErrorKind) {
            self.errors.push(RecordedError {
                kind,
                token_index: self.original_length - tokens.len(),
            });
        }

        pub fn error_contexts<'a>(&self, input: &'a str) -> Vec<ErrorContext<'a, ParseErrorKind>> {
            get_error_contexts(input, &self.errors)
        }
    }

    #[derive(Debug, Clone)]
    pub enum LexAndParseErrorKind<ParseErrorKind> {
        LexError(LexError),
        ParseError(ParseErrorKind),
    }

    #[derive(Debug)]
    struct PartialErrorContext<ParseErrorKind> {
        kind: LexAndParseErrorKind<ParseErrorKind>,
        column: usize,
    }

    pub struct ErrorContext<'a, ParseErrorKind> {
        partial: PartialErrorContext<ParseErrorKind>,
        line: &'a str,
    }

    impl<'a, ParseErrorKind: Clone> ErrorContext<'a, ParseErrorKind> {
        pub fn kind(&self) -> LexAndParseErrorKind<ParseErrorKind> {
            self.partial.kind.clone()
        }

        pub fn column(&self) -> usize {
            self.partial.column
        }

        pub fn line(&self) -> &'a str {
            self.line
        }
    }

    pub fn get_error_contexts<'a, ParseErrorKind: Clone + std::fmt::Debug>(
        mut input: &'a str,
        errors: &Vec<RecordedError<ParseErrorKind>>,
    ) -> Vec<ErrorContext<'a, ParseErrorKind>> {
        let mut contexts: Vec<ErrorContext<ParseErrorKind>> = Vec::with_capacity(errors.len());
        let mut partials: Vec<PartialErrorContext<ParseErrorKind>> = Vec::new();

        let errors = errors.iter().scan(0, |prev_token_index, error| {
            let new_error = RecordedError::<ParseErrorKind> {
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

            while is_skippable_whitespace(input.chars().next().unwrap()) {
                input = &input[1..];
                column += 1;
            }

            let original_input_len = input.len();
            match Token::extract(&mut input) {
                Token::Error(error) => {
                    partials.push(PartialErrorContext {
                        kind: LexAndParseErrorKind::LexError(error.clone()),
                        column,
                    });
                }
                _ => {
                    partials.push(PartialErrorContext {
                        kind: LexAndParseErrorKind::ParseError(error.kind.clone()),
                        column,
                    });
                }
            }

            column += original_input_len - input.len();
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
