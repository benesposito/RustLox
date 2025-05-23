use super::LookaheadLex;
use crate::{LexResult, Token};

#[derive(Debug, PartialEq)]
pub struct Identifier {
    pub name: String,
}

impl LookaheadLex for Identifier {
    fn is_kind(input: &str) -> bool {
        input
            .chars()
            .next()
            .expect("Expression is unexpectedly empty")
            .is_ascii_alphabetic()
    }

    fn extract(input: &mut &str) -> LexResult<Self> {
        for (i, c) in input.chars().enumerate() {
            if !c.is_alphanumeric() {
                let token = &input[..i];
                *input = &input[i..];
                return Ok(Identifier {
                    name: String::from(token),
                });
            }
        }

        let token = input.clone();
        *input = "";
        return Ok(Identifier {
            name: String::from(token),
        });
    }
}

impl From<Identifier> for Token {
    fn from(value: Identifier) -> Self {
        Token::Identifier(value)
    }
}
