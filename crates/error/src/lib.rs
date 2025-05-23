use lexer::{Token, is_skippable_whitespace};

pub enum Error<ErrorKind, PreviousErrorKind> {
    Error(ErrorKind),
    PreviousError(PreviousErrorKind),
}

#[derive(Debug)]
pub struct RecordedError<ErrorKind> {
    pub kind: ErrorKind,
    pub token_index: usize,
}

pub struct ErrorRecorder<ErrorKind> {
    //tokens: &'a TokenIterator,
    original_length: usize,
    errors: Vec<RecordedError<ErrorKind>>,
}

impl<ErrorKind: Clone + std::fmt::Debug> ErrorRecorder<ErrorKind> {
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

    pub fn errors(self) -> Errors<ErrorKind> {
        Errors::new(self.errors)
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

impl<'a, ErrorKind: Clone + std::fmt::Debug> ErrorContext<'a, ErrorKind> {
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

impl<'a, ErrorKind: Clone + std::fmt::Debug> std::fmt::Display for ErrorContext<'a, ErrorKind> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{:?}, {}", self.kind(), self.column())?;

        writeln!(f, "{}", self.line())?;
        write!(f, "{}^", String::from(" ").repeat(self.column()))?;

        Ok(())
    }
}

pub struct Errors<ErrorKind> {
    errors: Vec<RecordedError<ErrorKind>>,
}

impl<ErrorKind: Clone + std::fmt::Debug> Errors<ErrorKind> {
    pub fn new(errors: Vec<RecordedError<ErrorKind>>) -> Self {
        Errors { errors }
    }

    pub fn has_errors(&self) -> bool {
        !self.errors.is_empty()
    }

    pub fn error_kinds(&self) -> impl Iterator<Item = ErrorKind> {
        self.errors.iter().map(|error| error.kind.clone())
    }

    pub fn error_contexts<'a>(
        &self,
        input: &'a str,
    ) -> impl Iterator<Item = ErrorContext<'a, ErrorKind>> {
        get_error_contexts(input, self.errors.iter()).into_iter()
    }
}

fn get_error_contexts<'a, 'b, ErrorKind: Clone + std::fmt::Debug + 'b>(
    mut input: &'a str,
    errors: impl ExactSizeIterator<Item = &'b RecordedError<ErrorKind>>,
) -> Vec<ErrorContext<'a, ErrorKind>> {
    let mut contexts: Vec<ErrorContext<ErrorKind>> = Vec::with_capacity(errors.len());
    let mut partials: Vec<PartialErrorContext<ErrorKind>> = Vec::new();

    let errors = errors.scan(0, |prev_token_index, error| {
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
                Token::FixedToken(lexer::tokens::FixedToken::Newline) => {
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

        partials.push(PartialErrorContext {
            kind: error.kind,
            column,
        });

        let original_input_len = input.len();
        Token::extract(&mut input);
        column += original_input_len - input.len();
    }

    if !partials.is_empty() {
        loop {
            let original_input_len = input.len();
            match Token::extract(&mut input) {
                Token::FixedToken(lexer::tokens::FixedToken::Newline) => break,
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
