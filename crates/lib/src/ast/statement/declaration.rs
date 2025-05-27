use super::*;

#[derive(Debug)]
pub struct DeclarationList {
    pub list: Vec<Declaration>,
}

#[derive(Debug)]
pub enum Declaration {
    VariableDeclaration(String, Option<Expression>),
    Statement(Statement),
}

impl DeclarationList {
    pub fn parse<T: Iterator<Item = Token>>(
        parse_context: &mut ParseContext<ParseErrorKind, T>,
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
            Ok(DeclarationList {
                list: list.into_iter().map(|d| d.unwrap()).collect::<Vec<_>>(),
            })
        }
    }

    pub fn evaluate(&self, environment: &mut Environment) -> Result<(), RuntimeError> {
        todo!()
    }
}

impl Declaration {
    pub fn parse<T: Iterator<Item = Token>>(
        parse_context: &mut ParseContext<ParseErrorKind, T>,
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

    pub fn evaluate(&self, environment: &mut Environment) -> Result<(), RuntimeError> {
        match self {
            Declaration::Statement(statement) => statement.evaluate(environment),
            Declaration::VariableDeclaration(identifier, None) => {
                environment.declare_variable(identifier);
                Ok(())
            }
            Declaration::VariableDeclaration(identifier, Some(expression)) => {
                let value = expression.evaluate(environment)?;
                environment.define_variable(identifier, value);
                Ok(())
            }
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
