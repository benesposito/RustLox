use super::Lex;

#[derive(Debug)]
pub struct StringLiteral {
    pub value: String,
}

impl Lex for StringLiteral {
    fn extract(input: &mut &str) -> Option<Self> {
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
