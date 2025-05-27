pub enum Error<ErrorKind, PreviousErrorKind> {
    Error(ErrorKind),
    PreviousError(PreviousErrorKind),
}

#[derive(Debug)]
pub struct RecordedError<ErrorKind> {
    pub token_index: usize,
    pub kind: ErrorKind,
}

pub struct Errors<ErrorKind> {
    errors: Vec<RecordedError<ErrorKind>>,
}

impl<ErrorKind> Errors<ErrorKind>
where
    ErrorKind: Clone + std::fmt::Debug,
{
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
    ) -> impl Iterator<Item = ErrorContext<ErrorKind>> {
        get_error_contexts(input, self.errors.iter()).into_iter()
    }
}

pub struct ErrorRecorder<ErrorKind, I: Iterator<Item = lexer::Token>> {
    tokens: std::iter::Peekable<I>,
    original_length: usize,
    errors: Vec<RecordedError<ErrorKind>>,
}

pub struct DummyIterator {}
impl Iterator for DummyIterator {
    type Item = lexer::Token;
    fn next(&mut self) -> Option<Self::Item> {
        None
    }
}

impl<ErrorKind> ErrorRecorder<ErrorKind, DummyIterator> {
    pub fn new<I: ExactSizeIterator<Item = lexer::Token>>(
        tokens: I,
    ) -> ErrorRecorder<ErrorKind, impl Iterator<Item = lexer::Token>> {
        ErrorRecorder {
            original_length: tokens.len(),
            tokens: tokens
                .filter(|token| {
                    !matches!(
                        token,
                        lexer::Token::FixedToken(lexer::tokens::FixedToken::Newline)
                    )
                })
                .peekable(),
            errors: Vec::new(),
        }
    }
}

impl<ErrorKind, I> ErrorRecorder<ErrorKind, I>
where
    ErrorKind: Clone + std::fmt::Debug,
    I: Iterator<Item = lexer::Token>,
{
    pub fn tokens(&mut self) -> &mut std::iter::Peekable<impl Iterator<Item = lexer::Token>> {
        &mut self.tokens
    }

    pub fn record(&mut self, kind: ErrorKind) {
        self.errors.push(RecordedError {
            token_index: self.original_length - self.unfiltered_len(),
            kind,
        });
    }

    pub fn errors(self) -> Errors<ErrorKind> {
        Errors::new(self.errors)
    }

    fn unfiltered_len(&self) -> usize {
        self.tokens.size_hint().1.unwrap()
    }
}

#[derive(Debug)]
struct PartialErrorContext<ErrorKind> {
    kind: ErrorKind,
    column: usize,
}

pub struct ErrorContext<ErrorKind> {
    partial: PartialErrorContext<ErrorKind>,
    line: String,
}

impl<'a, ErrorKind: Clone + std::fmt::Debug> ErrorContext<ErrorKind> {
    pub fn kind(&self) -> ErrorKind {
        self.partial.kind.clone()
    }

    pub fn column(&self) -> usize {
        self.partial.column
    }

    pub fn line(&self) -> &str {
        &self.line
    }
}

impl<'a, ErrorKind: Clone + std::fmt::Debug> std::fmt::Display for ErrorContext<ErrorKind> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{:?}, {}", self.kind(), self.column())?;

        writeln!(f, "{}", self.line())?;
        write!(f, "{}^", String::from(" ").repeat(self.column()))?;

        Ok(())
    }
}

fn get_error_contexts<'a, ErrorKind: Clone + std::fmt::Debug + 'a>(
    mut input: &str,
    errors: impl ExactSizeIterator<Item = &'a RecordedError<ErrorKind>> + std::fmt::Debug,
) -> Vec<ErrorContext<ErrorKind>> {
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
    let mut token_index: usize = 0;

    for error in errors {
        for _ in 0..(error.token_index - 1) {
            token_index += skip_whitespace(&mut input);

            let original_input_len = input.len();
            match lexer::Token::extract(&mut input) {
                lexer::Token::FixedToken(lexer::tokens::FixedToken::Newline) => {
                    contexts.extend(partials.into_iter().map(|partial| ErrorContext {
                        partial,
                        line: start_of_line[0..token_index].replace('\t', "    "),
                    }));

                    partials = Vec::new();
                    start_of_line = input;
                    token_index = 0;
                }
                _ => {
                    token_index += original_input_len - input.len();
                }
            }
        }

        token_index += skip_whitespace(&mut input);

        partials.push(PartialErrorContext {
            kind: error.kind,
            column: token_index_to_column(start_of_line, token_index),
        });

        let original_input_len = input.len();
        lexer::Token::extract(&mut input);
        token_index += original_input_len - input.len();
    }

    if !partials.is_empty() {
        loop {
            let original_input_len = input.len();
            match lexer::Token::extract(&mut input) {
                lexer::Token::FixedToken(lexer::tokens::FixedToken::Newline) => break,
                _ => token_index += original_input_len - input.len(),
            }
        }

        contexts.extend(partials.into_iter().map(|partial| ErrorContext {
            partial,
            line: start_of_line[0..token_index].replace('\t', "    "),
        }));
    }

    contexts
}

fn skip_whitespace(input: &mut &str) -> usize {
    let mut skipped: usize = 0;

    for c in input.chars() {
        match c {
            c if lexer::is_skippable_whitespace(c) => skipped += 1,
            _ => {
                *input = &input[skipped..];
                break;
            }
        }
    }

    skipped
}

fn token_index_to_column(line: &str, token_index: usize) -> usize {
    let mut column = token_index;

    for c in line.chars().take(token_index) {
        if c == '\t' {
            column += 3;
        }
    }

    column
}
