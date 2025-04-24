use super::{Lex, LexImpl};

#[derive(Debug)]
pub struct StringLiteral {
    pub value: String,
}

impl LexImpl for StringLiteral {
    fn extract_impl(input: &mut &str) -> Option<Self> {
        for (i, c) in input.chars().enumerate().skip(1) {
            if c == '"' {
                let token = &input[1..i];
                *input = &input[i + 1..];
                return Some(StringLiteral {
                    value: String::from(token),
                });
            }
        }

        None
    }
}

impl Lex for StringLiteral {
    fn is_kind(input: &str) -> bool {
        input.starts_with("\"")
    }
}
