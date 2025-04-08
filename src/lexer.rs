#[derive(Clone, Debug)]
#[allow(dead_code)]
pub enum Token {
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
    Identifier(String),
    StringLiteral(String),
    NumericLiteral(f64),
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

    Newline,
}

/* TODO: measure relative frequencies of each token and order this
 * vector accordingly.
 *
 * Additionally, test whether checking shorter but less frequent tokens
 * before more frequent but longer tokens has a measurable difference.
 */
const TOKEN_MAP: &[(&str, Token)] = &[
    (">=", Token::GreaterEqual),
    (">", Token::Greater),
    ("<=", Token::LessEqual),
    ("<", Token::Less),
    ("==", Token::EqualEqual),
    ("=", Token::Equal),
    ("!=", Token::BangEqual),
    ("!", Token::Bang),
    ("(", Token::LeftParenthesis),
    (")", Token::RightParenthesis),
    ("{", Token::LeftBrace),
    ("}", Token::RightBrace),
    (",", Token::Comma),
    (".", Token::Dot),
    ("-", Token::Minus),
    ("+", Token::Plus),
    (";", Token::Semicolon),
    ("/", Token::ForwardSlash),
    ("*", Token::Asterisk),
    ("true", Token::True),
    ("false", Token::False),
    ("nil", Token::Nil),
    ("var", Token::Var),
    ("if", Token::If),
    ("else", Token::Else),
    ("for", Token::For),
    ("while", Token::While),
    ("fun", Token::Fun),
    ("return", Token::Return),
    ("class", Token::Class),
    ("this", Token::This),
    ("super", Token::Super),
    ("and", Token::And),
    ("or", Token::Or),
    ("print", Token::Print),
    ("\n", Token::Newline),
];

impl Token {
    fn extract(input: &mut &str) -> Option<Self> {
        if let Some(token) = Self::extract_fixed_token(input) {
            Some(token)
        } else if input
            .chars()
            .next()
            .expect("Expression is unexpectedly empty")
            .is_ascii_digit()
        {
            Self::extract_numeric_literal(input)
        } else if input.starts_with("\"") {
            Self::extract_string_literal(input)
        } else {
            Self::extract_identifier(input)
        }
    }

    fn extract_fixed_token(input: &mut &str) -> Option<Self> {
        for (token_string, token) in TOKEN_MAP {
            if let Some(rest_of_input) = input.strip_prefix(token_string) {
                *input = rest_of_input;
                return Some(token.clone());
            }
        }

        None
    }

    fn extract_numeric_literal(input: &mut &str) -> Option<Self> {
        let find_end_idx = || {
            for (i, c) in input.chars().enumerate() {
                if !(c.is_ascii_digit() || c == '.') {
                    return i;
                }
            }

            input.len()
        };

        let end = find_end_idx();
        let token = &input[..end];
        *input = &input[end..];

        Some(Self::NumericLiteral(
            token.parse().expect("Failed to parse NumericLiteral"),
        ))
    }

    fn extract_string_literal(input: &mut &str) -> Option<Self> {
        for (i, c) in input.chars().enumerate().skip(1) {
            if c == '"' {
                let token = &input[1..i];
                *input = &input[i + 1..];
                return Some(Self::StringLiteral(String::from(token)));
            }
        }

        None
    }

    fn extract_identifier(input: &mut &str) -> Option<Self> {
        for (i, c) in input.chars().enumerate() {
            if !c.is_alphanumeric() {
                let token = &input[..i];
                *input = &input[i..];
                return Some(Self::Identifier(String::from(token)));
            }
        }

        None
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
                Token::Newline => {
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
