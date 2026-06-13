use crate::parser::ParseError;

#[derive(PartialEq, Eq)]
pub enum ParsingMode {
    Expression,
    Code,
}
#[derive(Debug, Clone)]
pub enum AstNode {
    Expression(Box<Expression>),
    BlockCode(Vec<AstNode>),
}
impl AstNode {
    pub fn expr(self) -> Result<Expression, ParseError> {
        match self {
            Self::Expression(expr) => Ok(*expr),
            _ => Err(ParseError::NodeTypeError),
        }
    }
}
#[derive(Debug, Clone)]
pub struct Expression {
    pub(crate) operation: Option<Operation>,
    pub(crate) left: Vec<Value>,
    pub(crate) right: Vec<Value>,
}
#[derive(Debug, Clone)]
pub enum Value {
    Name(String),
    Number(isize),
    Bool(bool),
    Char(char),
    Expression(Expression),
}
impl Value {
    pub(crate) fn get_name(&self) -> Result<String, ()> {
        match &self {
            Self::Name(s) => Ok(s.clone()),
            _ => Err(()),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Operation {
    Comparison(Comparison),
    Computation(Computation),
    Logic(Logic),
    Vector(VectorOp),
    Call(String),
    Set,
}
#[derive(Debug, Clone)]
pub enum Part {
    Operation(Operation),
    Call,
    Keyword(Keyword),
    Value(Value),
}
#[derive(Debug, Clone, Copy)]
pub enum Keyword {
    True,
    False,
    Tab,
    Newline,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VectorOp {
    Pack,
    Unpack,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Comparison {
    Greater,
    Less,
    Equal,
    GreaterOrEqual,
    LessOrEqual,
    NotEqual,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Computation {
    Add,
    Sub,
    Mul,
    Div,
    Mod,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Logic {
    And,
    Or,
    Xor,
    Nand,
    Nor,
    Not,
}
