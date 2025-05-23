use super::LookaheadLex;
use crate::{LexError, LexResult, Token};

#[derive(Debug)]
pub struct StringLiteral {
    pub value: String,
}

impl LookaheadLex for StringLiteral {
    fn is_kind(input: &str) -> bool {
        input.starts_with("\"")
    }

    fn extract(input: &mut &str) -> LexResult<Self> {
        for (i, c) in input.chars().enumerate().skip(1) {
            match c {
                '"' => {
                    let token = &input[1..i];
                    *input = &input[i + 1..];
                    return Ok(StringLiteral {
                        value: String::from(token),
                    });
                }
                '\n' => {
                    *input = &input[i..];
                    println!("input: {:?}", input);
                    return Err(LexError::UnclosedString);
                }
                _ => (),
            }
        }

        Err(LexError::UnclosedString)
    }
}

impl From<StringLiteral> for Token {
    fn from(value: StringLiteral) -> Self {
        Token::StringLiteral(value)
    }
}
