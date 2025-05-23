pub mod expression;
pub mod statement;

use error::{Errors, RecordedError};
use statement::Statement;

use lexer::{LexError, Token, tokens::FixedToken};

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub enum ParseErrorKind {
    UnexpectedToken,
    UnmatchedParenthesis,
    ExpectedPrimaryExpression,
    ExpectedEndOfExpression,
    ExpectedSemicolon,
    ExpectedIdentifier,
}

type ParseResult<T> = Result<T, ParseErrorKind>;
pub type ParseError = RecordedError<ParseErrorKind>;

pub struct Ast {
    pub statements: Vec<Statement>,
}

impl Ast {
    pub fn parse(
        mut tokens: impl ExactSizeIterator<Item = Token>,
    ) -> Result<Self, Errors<ParseErrorKind>> {
        use error::ErrorRecorder;

        let mut statements: Vec<Statement> = Vec::new();
        let mut error_recorder = ErrorRecorder::<ParseErrorKind>::new(&tokens);

        loop {
            match Self::parse_enter(&mut tokens) {
                Some(Ok(statement)) => statements.push(statement),
                Some(Err(kind)) => {
                    error_recorder.record(&tokens, kind);

                    /* synchronize on semicolon */
                    while match tokens.next() {
                        Some(Token::FixedToken(FixedToken::Newline)) => false,
                        _ => true,
                    } {}
                }
                None => break,
            }
        }

        let errors = error_recorder.errors();

        if !errors.has_errors() {
            Ok(Ast { statements })
        } else {
            Err(errors)
        }
    }

    fn parse_enter(
        tokens: &mut impl Iterator<Item = Token>,
    ) -> Option<Result<Statement, ParseErrorKind>> {
        let mut tokens = tokens
            .filter(|token| !matches!(*token, Token::FixedToken(FixedToken::Newline)))
            .peekable();

        if tokens.peek().is_some() {
            Some(Statement::parse(&mut tokens))
        } else {
            None
        }
    }
}
