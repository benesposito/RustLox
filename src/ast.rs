pub mod expression;
pub mod statement;

use crate::lexer::Token;
use statement::Statement;

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub enum ParseErrorKind {
    UnmatchedParenthesis,
    ExpectedPrimaryExpressionBefore,
    ExpectedEndOfExpression,
    ExpectedSemicolon,
    ExpectedIdentifier,
}

#[derive(Debug)]
#[allow(dead_code)]
pub struct ParseError {
    pub kind: ParseErrorKind,
    pub token_index: usize,
}

type ParseResult<T> = Result<T, ParseErrorKind>;

pub struct Ast {
    pub statements: Vec<Statement>,
}

impl Ast {
    pub fn parse(
        mut tokens: impl ExactSizeIterator<Item = Token>,
    ) -> Result<Self, Vec<ParseError>> {
        let orig_len = tokens.len();
        let mut statements: Vec<Statement> = Vec::new();
        let mut errors: Vec<ParseError> = Vec::new();

        loop {
            match Self::parse_enter(&mut tokens) {
                Some(Ok(statement)) => statements.push(statement),
                Some(Err(kind)) => errors.push(ParseError {
                    kind,
                    token_index: orig_len - tokens.len(),
                }),
                None => break,
            }
        }

        if errors.is_empty() {
            Ok(Ast { statements })
        } else {
            Err(errors)
        }
    }

    fn parse_enter(
        tokens: &mut impl Iterator<Item = Token>,
    ) -> Option<Result<Statement, ParseErrorKind>> {
        let mut tokens = tokens
            .filter(|token| !matches!(*token, Token::Newline))
            .peekable();

        if tokens.peek().is_some() {
            Some(Statement::parse(&mut tokens))
        } else {
            None
        }
    }
}
