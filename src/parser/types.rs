use crate::parser::ParseError;

#[derive(PartialEq, Eq)]
pub enum ParsingMode {
    Expression,
    Code,
}
#[derive(Debug, Clone)]
pub enum AstNode {
    Expression(ExpressionNode),
    BlockCode(Vec<AstNode>),
}
#[derive(Debug, Clone)]
pub enum ExpressionNode {
    Multi(Box<Folder>),
    Binary(Box<BinaryExpression>),
}
impl AstNode {
    pub fn expr(self) -> Result<ExpressionNode, ParseError> {
        match self {
            Self::Expression(expr) => Ok(expr),
            _ => Err(ParseError::UnexpectedBlock),
        }
    }
}
#[derive(Debug, Clone)]
pub struct BinaryExpression {
    pub(crate) operation: Option<Operation>,
    pub(crate) left: Value,
    pub(crate) right: Value,
}
#[derive(Debug, Clone)]
pub struct Folder {
    pub(crate) operation: Option<Operation>,
    pub(crate) left: Vec<Value>,
    pub(crate) right: Vec<Value>,
}
#[derive(Debug, Clone)]
pub enum Value {
    Name(String),
    Literal(String),
    Number(isize),
    Bool(bool),
    Char(char),
    Expression(ExpressionNode),
}
impl Value {
    pub(crate) fn get_name(&self) -> Result<String, Value> {
        match self {
            Self::Name(s) => Ok(s.clone()),
            value => Err(value.clone()),
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
    MultiOperation(Operation),
    Operation(Operation),
    Call,
    Constant(Constant),
    Value(Value),
}
#[derive(Debug, Clone, Copy)]
pub enum Constant {
    True,
    False,
    Tab,
    Newline,
    Space,
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
