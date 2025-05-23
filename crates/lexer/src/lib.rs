pub mod tokens;

use tokens::*;

#[derive(Debug, Clone)]
pub enum LexError {
    NoTokenKind,
    NumericContainsAlpha,
    UnclosedString,
}

type LexResult<T> = std::result::Result<T, LexError>;

#[derive(Debug)]
pub enum Token {
    FixedToken(FixedToken),
    Identifier(Identifier),
    StringLiteral(StringLiteral),
    NumericLiteral(NumericLiteral),
    Error(LexError),
}

impl Token {
    pub fn extract(input: &mut &str) -> Self {
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
            let new_input = input.trim_start_matches(|c| !is_skippable_whitespace(c));
            let token = &input[..new_input.len()];
            *input = new_input;
            //Ok(Token::Error(LexError::NoTokenKind, String::from(input[..new_input.len()])))
            Token::Error(LexError::NoTokenKind)
        }
    }
}

impl<T: tokens::LookaheadLex> From<LexResult<T>> for Token
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

pub fn is_skippable_whitespace(c: char) -> bool {
    c.is_whitespace() && c != '\n'
}
