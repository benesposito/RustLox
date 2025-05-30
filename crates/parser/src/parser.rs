use lexer::Token;

#[derive(Debug, Clone)]
pub enum ParseErrorKind {
    UnexpectedToken,
    UnmatchedParenthesis,
    ExpectedPrimaryExpression,
    ExpectedEndOfExpression,
    ExpectedSemicolon,
    ExpectedIdentifier,
}

pub struct Ast {
    pub program: crate::grammar::Program,
}

impl Ast {
    pub fn new(
        tokens: impl ExactSizeIterator<Item = Token>,
    ) -> Result<Self, error::Errors<ParseErrorKind>> {
        let mut parse_context = ParseContext::<ParseErrorKind, _>::new(tokens);

        match crate::grammar::Program::parse(&mut parse_context) {
            Ok(program) => Ok(Ast { program }),
            Err(_) => Err(parse_context.errors()),
        }
    }
}

impl std::fmt::Display for Ast {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.program)
    }
}

pub enum ShouldSynchronize {
    Yes,
    No,
}

pub type ParseResult<T> = Result<T, ShouldSynchronize>;
pub type ParseError = error::RecordedError<ParseErrorKind>;

pub struct ParseContext<ErrorKind, I>
where
    ErrorKind: Clone + std::fmt::Debug,
    I: Iterator<Item = Token>,
{
    recorder: error::ErrorRecorder<ErrorKind, I>,
}

impl<ErrorKind: Clone + std::fmt::Debug> ParseContext<ErrorKind, error::DummyIterator> {
    pub fn new<I: ExactSizeIterator<Item = Token>>(
        tokens: I,
    ) -> ParseContext<ErrorKind, impl Iterator<Item = Token>> {
        ParseContext {
            recorder: error::ErrorRecorder::new(tokens),
        }
    }
}

impl<ErrorKind, I> ParseContext<ErrorKind, I>
where
    ErrorKind: Clone + std::fmt::Debug,
    I: Iterator<Item = Token>,
{
    pub fn tokens(&mut self) -> &mut std::iter::Peekable<impl Iterator<Item = Token>> {
        self.recorder.tokens()
    }

    pub fn record_error(&mut self, kind: ErrorKind) {
        self.recorder.record(kind)
    }

    pub fn errors(self) -> error::Errors<ErrorKind> {
        self.recorder.errors()
    }
}

fn synchronize_default(tokens: &mut impl Iterator<Item = Token>) {
    synchronize(tokens, lexer::tokens::FixedToken::Semicolon)
}

fn synchronize(tokens: &mut impl Iterator<Item = Token>, token: lexer::tokens::FixedToken) {
    tokens.find(|t| {
        matches!(t, Token::FixedToken(fixed_token) if std::mem::discriminant(fixed_token) == std::mem::discriminant(&token))
    });
}
