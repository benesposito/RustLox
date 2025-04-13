use super::Lex;

#[derive(Debug)]
pub struct Identifier {
    pub name: String,
}

impl Lex for Identifier {
    fn extract(input: &mut &str) -> Option<Self> {
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
