mod fixed_token;
mod identifier;
mod numeric_literal;
mod string_literal;

pub use fixed_token::FixedToken;
pub use identifier::Identifier;
pub use numeric_literal::NumericLiteral;
pub use string_literal::StringLiteral;

pub trait LookaheadLex: std::fmt::Debug {
    fn is_kind(input: &str) -> bool;

    /// Consume and return a token from a string slice.
    ///
    /// # Examples
    /// ```
    /// # use lib::lexer::*;
    /// let mut code = "var x = 5;";
    ///
    /// let token = FixedToken::extract(&mut code).unwrap();
    /// assert!(matches!(token, FixedToken::Var));
    ///
    /// let token = Identifier::extract(&mut code).unwrap();
    /// assert_eq!(token, Identifier{name: String::from("x")});
    ///
    /// let token = FixedToken::extract(&mut code).unwrap();
    /// assert!(matches!(token, FixedToken::Equal));
    ///
    /// let token = NumericLiteral::extract(&mut code).unwrap();
    /// assert_eq!(token, NumericLiteral{value: 5f64});
    ///
    /// let token = FixedToken::extract(&mut code).unwrap();
    /// assert!(matches!(token, FixedToken::Semicolon));
    /// ```
    fn extract(input: &mut &str) -> crate::LexResult<Self>
    where
        Self: Sized;
}
