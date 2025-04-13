use super::{Lex, LexImpl};

#[derive(Debug, PartialEq)]
pub struct NumericLiteral {
    pub value: f64,
}

impl NumericLiteral {
    pub fn new(value: f64) -> NumericLiteral {
        NumericLiteral { value }
    }
}

impl LexImpl for NumericLiteral {
    fn extract_impl(input: &mut &str) -> Option<Self> {
        let end = (|| {
            let mut chars = input.chars().peekable();
            let has_sign = matches!(chars.peek(), Some('-'));

            if has_sign {
                chars.next();
            }

            for (i, c) in chars.enumerate() {
                if !(c.is_ascii_digit() || c == '.') {
                    return i + if has_sign { 1 } else { 0 };
                }
            }

            input.len()
        })();

        let token = &input[..end];
        *input = &input[end..];

        Some(NumericLiteral::new(
            token.parse().expect("Failed to parse NumericLiteral"),
        ))
    }
}

impl Lex for NumericLiteral {}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::prelude::Distribution;
    use rand::prelude::IndexedRandom;

    fn generate_numeric_literal(rng: &mut impl rand::Rng) -> (String, NumericLiteral) {
        let range = rand_distr::Frechet::new(0.0, 2.0, 0.1).unwrap();
        let value = range.sample(rng) * (*[-1.0, 1.0].choose(rng).unwrap());

        println!("{:?}", (value.to_string(), NumericLiteral { value }));
        (value.to_string(), NumericLiteral { value })
    }

    #[test]
    fn test_extract() {
        let mut rng = rand::rng();

        let mut should = std::collections::HashMap::from([
            (String::from("0"), NumericLiteral::new(0f64)),
            (String::from("-0"), NumericLiteral::new(0f64)),
            (String::from("123.456"), NumericLiteral::new(123.456)),
            (String::from("-123.456"), NumericLiteral::new(-123.456)),
        ]);

        for _ in 0..10 {
            should.extend([generate_numeric_literal(&mut rng)]);
        }

        let should_not = std::collections::HashMap::from([
            (String::from("0"), NumericLiteral::new(0.5f64)),
            (String::from("1234"), NumericLiteral::new(1234.1f64)),
        ]);

        let should = should.into_iter().map(|pair| (true, pair.0, pair.1));
        let should_not = should_not.into_iter().map(|pair| (false, pair.0, pair.1));
        let records = should.chain(should_not);

        for record in records {
            assert!(
                record.0
                    == matches!(NumericLiteral::extract(&mut record.1.as_str()), Some(actual) if actual.value == record.2.value)
            );
        }
    }
}
