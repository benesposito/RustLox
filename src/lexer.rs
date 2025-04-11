trait Lex {
    fn extract(input: &mut &str) -> Option<Self>
    where
        Self: Sized;
}

#[derive(Clone, Debug)]
pub enum FixedToken {
    /* Symbols */
    LeftParenthesis,
    RightParenthesis,
    LeftBrace,
    RightBrace,
    Plus,
    Minus,
    Asterisk,
    ForwardSlash,
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,
    Comma,
    Dot,
    Semicolon,

    /* Literals */
    True,
    False,
    Nil,

    /* Keywords */
    Var,

    If,
    Else,
    For,
    While,

    Fun,
    Return,

    Class,
    This,
    Super,

    And,
    Or,

    Print,

    /* Misc */
    Newline,
}

#[derive(Debug)]
pub struct Identifier {
    pub name: String,
}

#[derive(Debug)]
pub struct StringLiteral {
    pub value: String,
}

#[derive(Debug)]
pub struct NumericLiteral {
    pub value: f64,
}

#[derive(Debug)]
pub enum Token {
    FixedToken(FixedToken),
    Identifier(Identifier),
    StringLiteral(StringLiteral),
    NumericLiteral(NumericLiteral),
}

/* TODO: measure relative frequencies of each token and order this
 * vector accordingly.
 *
 * Additionally, test whether checking shorter but less frequent tokens
 * before more frequent but longer tokens has a measurable difference.
 *
 * Note: longer fixed tokens which contain tokens within them (>=, <=, ==, !=)
 * must come before their shorter subtokens in order to be parsed correctly.
 *
 * Note: Minus is not in this map as it is separately checked to avoid the cost of determining
 * whether it is a part of a numeric literal.
 */
const FIXED_TOKEN_MAP: &[(&str, FixedToken)] = &[
    (">=", FixedToken::GreaterEqual),
    (">", FixedToken::Greater),
    ("<=", FixedToken::LessEqual),
    ("<", FixedToken::Less),
    ("==", FixedToken::EqualEqual),
    ("=", FixedToken::Equal),
    ("!=", FixedToken::BangEqual),
    ("!", FixedToken::Bang),
    ("(", FixedToken::LeftParenthesis),
    (")", FixedToken::RightParenthesis),
    ("{", FixedToken::LeftBrace),
    ("}", FixedToken::RightBrace),
    (",", FixedToken::Comma),
    (".", FixedToken::Dot),
    ("+", FixedToken::Plus),
    (";", FixedToken::Semicolon),
    ("/", FixedToken::ForwardSlash),
    ("*", FixedToken::Asterisk),
    ("true", FixedToken::True),
    ("false", FixedToken::False),
    ("nil", FixedToken::Nil),
    ("var", FixedToken::Var),
    ("if", FixedToken::If),
    ("else", FixedToken::Else),
    ("for", FixedToken::For),
    ("while", FixedToken::While),
    ("fun", FixedToken::Fun),
    ("return", FixedToken::Return),
    ("class", FixedToken::Class),
    ("this", FixedToken::This),
    ("super", FixedToken::Super),
    ("and", FixedToken::And),
    ("or", FixedToken::Or),
    ("print", FixedToken::Print),
    ("\n", FixedToken::Newline),
];

impl Lex for FixedToken {
    fn extract(input: &mut &str) -> Option<Self> {
        for (token_string, token) in FIXED_TOKEN_MAP {
            if let Some(rest_of_input) = input.strip_prefix(token_string) {
                *input = rest_of_input;
                return Some(token.clone());
            }
        }

        /* Check for minus token only if it is not followed by a numeric. Doing
         * this after checking the map keeps the hot path fast, since otherwise
         * we'd have to if an if check on whether the token is a minus in every
         * iteration.
         */
        let mut chars = input.chars();

        if matches!(chars.next(), Some('-')) {
            match chars.next() {
                Some(c) if !c.is_numeric() => {
                    let rest_of_input = input.strip_prefix('-').unwrap();
                    *input = rest_of_input;
                    return Some(FixedToken::Minus);
                }
                _ => (),
            }
        }

        None
    }
}

impl NumericLiteral {
    pub fn new(value: f64) -> NumericLiteral {
        NumericLiteral { value }
    }
}

impl Lex for NumericLiteral {
    fn extract(input: &mut &str) -> Option<Self> {
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

impl Token {
    fn extract(input: &mut &str) -> Option<Self> {
        if let Some(token) = FixedToken::extract(input) {
            Some(Token::FixedToken(token))
        } else if input
            .chars()
            .next()
            .expect("Expression is unexpectedly empty")
            .is_ascii_digit()
        {
            Some(Token::NumericLiteral(NumericLiteral::extract(input)?))
        } else if input.starts_with("\"") {
            Some(Token::StringLiteral(StringLiteral::extract(input)?))
        } else {
            Some(Token::Identifier(Identifier::extract(input)?))
        }
    }
}

pub fn tokenize(mut input: &str) -> Option<Vec<Token>> {
    let mut tokens: Vec<Token> = Vec::new();

    input = input.trim_start_matches(|c: char| c.is_whitespace() && c != '\n');

    while !input.is_empty() {
        match Token::extract(&mut input) {
            Some(token) => {
                tokens.push(token);
            }
            None => return None,
        }

        input = input.trim_start_matches(|c: char| c.is_whitespace() && c != '\n');
    }

    Some(tokens)
}

pub struct ErrorContext<'a> {
    pub kind: crate::ast::ParseErrorKind,
    pub line: &'a str,
    pub column: usize,
}

#[derive(Debug)]
struct PartialErrorContext {
    kind: crate::ast::ParseErrorKind,
    column: usize,
}

pub fn get_error_contexts<'a>(
    mut input: &'a str,
    errors: &Vec<crate::ast::ParseError>,
) -> Vec<ErrorContext<'a>> {
    let mut contexts: Vec<ErrorContext> = Vec::with_capacity(errors.len());
    let mut partials: Vec<PartialErrorContext> = Vec::new();

    let errors = errors.iter().scan(0, |prev_token_index, error| {
        let new_error = crate::ast::ParseError {
            kind: error.kind.clone(),
            token_index: error.token_index - *prev_token_index,
        };

        *prev_token_index = error.token_index;
        Some(new_error)
    });

    let mut start_of_line = input;
    let mut column: usize = 0;

    for error in errors {
        for _ in 0..error.token_index {
            while input.chars().next().unwrap().is_whitespace()
                && input.chars().next().unwrap() != '\n'
            {
                input = &input[1..];
                column += 1;
            }

            let original_input_len = input.len();
            match Token::extract(&mut input).expect("Tokens have already been validated") {
                Token::FixedToken(FixedToken::Newline) => {
                    contexts.extend(partials.into_iter().map(|partial| ErrorContext {
                        kind: partial.kind.clone(),
                        line: &start_of_line[0..column],
                        column: partial.column,
                    }));

                    partials = Vec::new();
                    start_of_line = input;
                    column = 0;
                }
                _ => {
                    column += original_input_len - input.len();
                }
            }
        }

        partials.push(PartialErrorContext {
            kind: error.kind.clone(),
            column,
        });
    }

    contexts.extend(partials.into_iter().map(|partial| ErrorContext {
        kind: partial.kind.clone(),
        line: &start_of_line[0..column],
        column: partial.column,
    }));

    contexts
}

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
    fn test_fixed_token() {
        for &(mut token_string, ref expected_token) in FIXED_TOKEN_MAP {
            assert!(matches!(
                FixedToken::extract(&mut token_string),
                Some(actual_token) if std::mem::discriminant(&actual_token) == std::mem::discriminant(&expected_token)
            ));
        }
    }

    #[test]
    fn test_numeric_literal() {
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
