mod declaration;
mod expression;
mod statement;

#[derive(Debug)]
pub struct Program {
    pub declarations: Vec<Declaration>,
}

#[derive(Debug)]
pub enum Declaration {
    VariableDeclaration(VariableDeclaration),
    Statement(Statement),
}

#[derive(Debug)]
pub struct VariableDeclaration {
    pub identifier: String,
    pub value: Option<Expression>,
}

#[derive(Debug)]
pub enum Statement {
    ExpressionStatement(Expression),
    IfStatement {
        condition: Expression,
        then: Box<Statement>,
        else_: Option<Box<Statement>>,
    },
    PrintStatement(Expression),
    WhileStatement {
        condition: Expression,
        body: Box<Statement>,
    },
    Block(Block),
}

#[derive(Debug)]
pub struct Block {
    pub statements: Vec<Declaration>,
}

#[derive(Debug)]
pub enum Expression {
    Assignment{
        identifier: String,
        value: Box<Expression>,
    },
    Unary(Unary),
    Binary(Binary),
    Primary(Primary),
}

#[derive(Debug)]
pub struct Unary {
    pub operator: UnaryOperator,
    pub right: Box<Expression>,
}

#[derive(Debug)]
pub struct Binary {
    pub left: Box<Expression>,
    pub operator: BinaryOperator,
    pub right: Box<Expression>,
}

#[derive(Debug)]
pub enum BinaryOperator {
    Equality,
    Inequality,
    GreaterThan,
    GreaterThanOrEqualTo,
    LessThan,
    LessThanOrEqualTo,
    And,
    Or,
    Addition,
    Subtraction,
    Multiplication,
    Division,
}

#[derive(Debug)]
pub enum UnaryOperator {
    Negate,
    Not,
}

#[derive(Debug)]
pub enum Primary {
    Call {
        callable: Box<Primary>,
        arguments: Vec<Expression>,
    },
    True,
    False,
    Nil,
    Number(f64),
    String_(String),
    Identifier(String),
    Grouping(Box<Expression>),
}
