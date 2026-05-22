#[derive(PartialEq, Eq)]
pub(crate) enum ParsingMode {
    Expression,
    Code,
}
#[derive(Debug, Clone)]
pub(crate) enum AstNode {
    Expression(Expression),
    BlockCode(Vec<AstNode>),
}
impl AstNode {
    pub fn expr(self) -> Result<Expression, ()> {
        match self {
            Self::Expression(expr) => Ok(expr),
            _ => Err(()),
        }
    }
}
#[derive(Debug, Clone)]
pub(crate) struct Expression {
    pub(crate) operation: Option<Operation>,
    pub(crate) left: Vec<Value>,
    pub(crate) right: Vec<Value>,
}
#[derive(Debug, Clone)]
pub(crate) enum Value {
    Name(String),
    Number(isize),
    Bool(bool),
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
#[derive(Debug, Clone)]
pub(crate) enum Operation {
    Comparison(Comparison),
    Computation(Computation),
    Logic(Logic),
    Vector(VectorOp),
    Call(String),
    Set,
}
#[derive(Debug, Clone)]
pub(super) enum Part {
    Operation(Operation),
    Expression(Expression),
    Call,
    Name(String),
    Keyword(Keyword),
    Number(isize),
}
#[derive(Debug, Clone, Copy)]
pub(super) enum Keyword {
    If,
    Else,
    Redo,
    End,
    True,
    False,
}
#[derive(Debug, Clone, Copy)]
pub(crate) enum VectorOp {
    Pack,
    Unpack,
}
#[derive(Debug, Clone, Copy)]
pub(crate) enum Comparison {
    Greater,
    Less,
    Equal,
    GreaterOrEqual,
    LessOrEqual,
    NotEqual,
}
#[derive(Debug, Clone, Copy)]
pub(crate) enum Computation {
    Add,
    Sub,
    Mul,
    Div,
    Mod,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Logic {
    And,
    Or,
    Xor,
    Nand,
    Nor,
    Not,
}
