use crate::grammar::*;
use crate::parser::*;

use lexer::{Token, tokens::FixedToken};

impl Program {
    pub fn parse<T: Iterator<Item = Token>>(
        parse_context: &mut ParseContext<T>,
    ) -> ParseResult<Self> {
        let mut list: Vec<Option<Declaration>> = Vec::new();

        loop {
            match Declaration::parse(parse_context) {
                Ok(declaration) => list.push(Some(declaration)),
                Err(should_synchronize) => {
                    list.push(None);

                    if let ShouldSynchronize::Yes = should_synchronize {
                        loop {
                            match parse_context.tokens().next() {
                                Some(Token::FixedToken(FixedToken::Semicolon)) | None => break,
                                _ => (),
                            };
                        }
                    }
                }
            }

            if parse_context.tokens().peek().is_none() {
                break;
            }
        }

        if list.iter().any(|x| x.is_none()) {
            Err(ShouldSynchronize::No)
        } else {
            Ok(Self {
                declarations: list.into_iter().map(|d| d.unwrap()).collect::<Vec<_>>(),
            })
        }
    }
}

impl std::fmt::Display for Program {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for declaration in &self.declarations {
            writeln!(f, "{declaration}")?;
        }

        Ok(())
    }
}

impl Declaration {
    pub fn parse<T: Iterator<Item = Token>>(
        parse_context: &mut ParseContext<T>,
    ) -> ParseResult<Self> {
        match parse_context.tokens().peek().expect("Expected tokens") {
            Token::FixedToken(FixedToken::Var) => {
                parse_context.tokens().next();

                let identifier = match parse_context.tokens().next() {
                    Some(Token::Identifier(identifier)) => identifier,
                    _ => {
                        parse_context.record_error(ParseErrorKind::ExpectedIdentifier);
                        return Err(ShouldSynchronize::Yes);
                    }
                };

                let declaration = match parse_context.tokens().peek() {
                    Some(Token::FixedToken(FixedToken::Equal)) => {
                        parse_context.tokens().next();
                        Declaration::VariableDeclaration(
                            identifier.name,
                            Some(Expression::parse(parse_context)?),
                        )
                    }
                    _ => Declaration::VariableDeclaration(identifier.name, None),
                };

                match parse_context.tokens().next() {
                    Some(Token::FixedToken(FixedToken::Semicolon)) => Ok(declaration),
                    _ => {
                        parse_context.record_error(ParseErrorKind::ExpectedSemicolon);
                        Err(ShouldSynchronize::Yes)
                    }
                }
            }
            _ => Ok(Declaration::Statement(Statement::parse(parse_context)?)),
        }
    }
}

impl std::fmt::Display for Declaration {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Declaration::VariableDeclaration(identifier, rhs) => {
                write!(f, "(declare-variable {} {:?})", identifier, rhs)
            }
            Declaration::Statement(statement) => statement.fmt(f),
        }
    }
}
