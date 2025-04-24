use super::{Lex, LexImpl};

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

impl LexImpl for FixedToken {
    fn extract_impl(input: &mut &str) -> Option<Self> {
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

impl Lex for FixedToken {
    fn is_kind(_: &str) -> bool {
        todo!("Find way of implementing in a way that caches the result. For
        now, use 'if let Some(token) = FixedToken::extract(input)'");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract() {
        for &(mut token_string, ref expected_token) in FIXED_TOKEN_MAP {
            assert!(matches!(
                FixedToken::extract(&mut token_string),
                Some(actual_token) if std::mem::discriminant(&actual_token) == std::mem::discriminant(&expected_token)
            ));
        }
    }
}
