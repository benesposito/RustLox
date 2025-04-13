use super::{Lex, LexImpl};

#[derive(Debug, PartialEq)]
pub struct Identifier {
    pub name: String,
}

impl LexImpl for Identifier {
    fn extract_impl(input: &mut &str) -> Option<Self> {
        for (i, c) in input.chars().enumerate() {
            if !c.is_alphanumeric() {
                let token = &input[..i];
                *input = &input[i..];
                return Some(Identifier {
                    name: String::from(token),
                });
            }
        }

        None
    }
}

impl Lex for Identifier {}
