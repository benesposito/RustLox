pub mod expression;
pub mod statement;

use crate::lexer::{FixedToken, Token, error_context::RecordedError};
use statement::Statement;

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub enum ParseErrorKind {
    UnexpectedToken,
    UnmatchedParenthesis,
    ExpectedPrimaryExpressionBefore,
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
    ) -> Result<Self, Vec<ParseError>> {
        use crate::lexer::error_context::ErrorRecorder;

        let mut statements: Vec<Statement> = Vec::new();
        let mut error_recorder = ErrorRecorder::<ParseErrorKind>::new(&tokens);

        loop {
            match Self::parse_enter(&mut tokens) {
                Some(Ok(statement)) => statements.push(statement),
                Some(Err(kind)) => error_recorder.record(&tokens, kind),
                None => break,
            }
        }

        if error_recorder.errors.is_empty() {
            Ok(Ast { statements })
        } else {
            Err(error_recorder.errors)
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
