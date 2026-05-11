#[derive(PartialEq, Eq)]
pub enum ParsingMode {
    Expression,
    Code,
}
#[derive(Debug, Clone)]
pub enum AstNode {
    Expression(Expression),
    BlockCode(Vec<AstNode>),
}
#[derive(Debug, Clone)]
pub struct Expression {
    pub operation: Option<Operation>,
    pub left: Vec<Value>,
    pub right: Vec<Value>,
}
#[derive(Debug, Clone)]
pub enum Value {
    Name(String),
    Number(isize),
    Expression(Expression),
}
impl Value {
    pub fn get_name(&self) -> Result<String, ()> {
        match &self {
            Self::Name(s) => Ok(s.clone()),
            _ => Err(()),
        }
    }
}
#[derive(Debug, Clone)]
pub enum Operation {
    Comparison(Comparison),
    Computation(Computation),
    Logic(Logic),
    Vector(VectorOp),
    Call(String),
    Set,
}
#[derive(Debug, Clone)]
enum Part {
    Operation(Operation),
    Node(AstNode),
    Call,
    Name(String),
    Number(isize),
}
#[derive(Debug, Clone, Copy)]
pub enum VectorOp {
    Pack,
    Unpack,
}
#[derive(Debug, Clone, Copy)]
pub enum Comparison {
    Greater,
    Less,
    Equal,
    GreaterOrEqual,
    LessOrEqual,
    NotEqual,
}
#[derive(Debug, Clone, Copy)]
pub enum Computation {
    Add,
    Sub,
    Mul,
    Div,
    Mod,
}
#[derive(Debug, Clone, Copy)]
pub enum Logic {
    And,
    Or,
    Xor,
    Nand,
    Nor,
    Not,
}
pub fn astify(
    tokens: &Vec<String>,
    block_type: ParsingMode,
    index: &mut usize,
) -> Result<AstNode, String> {
    let mut buffer: Vec<Part> = Vec::new();
    let mut nodes: Vec<AstNode> = Vec::new();
    while *index < tokens.len() {
        match tokens[*index].as_str() {
            "(" => {
                *index += 1;
                buffer.push(Part::Node(
                    astify(tokens, ParsingMode::Expression, index).unwrap(),
                ));
            }
            "{" => {
                *index += 1;
                nodes.push(astify(tokens, ParsingMode::Code, index).unwrap())
            }
            ")" => {
                if block_type == ParsingMode::Expression {
                    let node = parse_expression(&buffer);
                    buffer.clear();
                    // println!("parsed expression: {:?}", node);
                    return Ok(AstNode::Expression(node));
                }
            }
            "}" => {
                if block_type == ParsingMode::Code {
                    return Ok(AstNode::BlockCode(nodes));
                }
            }
            ";" => match block_type {
                ParsingMode::Expression => {
                    let expr = Part::Node(AstNode::Expression(parse_expression(&buffer)));
                    buffer.clear();
                    buffer.push(expr);
                }
                ParsingMode::Code => {
                    if !buffer.is_empty() {
                        let node = parse_expression(&buffer);
                        buffer.clear();
                        // println!("parsed expression: {:?}", node);
                        nodes.push(AstNode::Expression(node));
                    }
                }
            },
            "=" => buffer.push(Part::Operation(Operation::Set)),
            ":" => buffer.push(Part::Call),
            "+" => buffer.push(Part::Operation(Operation::Computation(Computation::Add))),
            "-" => buffer.push(Part::Operation(Operation::Computation(Computation::Sub))),
            "*" => buffer.push(Part::Operation(Operation::Computation(Computation::Mul))),
            "/" => buffer.push(Part::Operation(Operation::Computation(Computation::Div))),
            "%" => buffer.push(Part::Operation(Operation::Computation(Computation::Mod))),
            "|" => buffer.push(Part::Operation(Operation::Logic(Logic::Or))),
            "&" => buffer.push(Part::Operation(Operation::Logic(Logic::And))),
            "^" => buffer.push(Part::Operation(Operation::Logic(Logic::Xor))),
            "!" => buffer.push(Part::Operation(Operation::Logic(Logic::Not))),
            "==" | "!^" => buffer.push(Part::Operation(Operation::Comparison(Comparison::Equal))),
            "!=" => buffer.push(Part::Operation(Operation::Comparison(Comparison::NotEqual))),
            ">" => buffer.push(Part::Operation(Operation::Comparison(Comparison::Greater))),
            "<" => buffer.push(Part::Operation(Operation::Comparison(Comparison::Less))),
            ">=" | "=>" | "!<" | "<!" => buffer.push(Part::Operation(Operation::Comparison(
                Comparison::GreaterOrEqual,
            ))),
            "<=" | "=<" | "!>" | ">!" => buffer.push(Part::Operation(Operation::Comparison(
                Comparison::LessOrEqual,
            ))),
            "!&" => buffer.push(Part::Operation(Operation::Logic(Logic::Nand))),
            "!|" => buffer.push(Part::Operation(Operation::Logic(Logic::Nor))),
            ",," => buffer.push(Part::Operation(Operation::Vector(VectorOp::Pack))),
            ".." => buffer.push(Part::Operation(Operation::Vector(VectorOp::Unpack))),
            x if x.chars().all(|c| c.is_numeric())
                | (x.starts_with("-") && x[1..].chars().all(|c| c.is_numeric())) =>
            {
                buffer.push(Part::Number(x.parse::<isize>().unwrap()));
            }
            x if x.chars().all(|c| c.is_alphanumeric() || c == '_') => {
                buffer.push(Part::Name(x.to_string()));
            }

            x => panic!("unexpected token: \"{x}\""),
        }
        // println!("buffer: {:?}", buffer);
        // println!("index: {}", *index);
        *index += 1;
    }
    if !buffer.is_empty() {
        for part in buffer {
            match part {
                Part::Node(n) => nodes.push(n),
                _ => panic!("unexpected part in buffer at end of tokens: {:?}", part),
            }
        }
    }
    match block_type {
        ParsingMode::Expression => panic!("unexpected end of expression"),
        ParsingMode::Code => Ok(AstNode::BlockCode(nodes)),
    }
}
fn parse_expression(buffer: &Vec<Part>) -> Expression {
    let mut idx = 0;
    // println!("parsing expression from: {:?}", buffer);
    let mut left: Vec<Part> = Vec::new();
    let mut right: Vec<Part> = Vec::new();
    let mut operation: Option<Operation> = None;
    while idx < buffer.len() {
        match buffer[idx].clone() {
            Part::Operation(op) => {
                if operation.is_none() {
                    operation = Some(op);
                } else {
                    panic!("Second operation inside one paren found while parsing ")
                }
            }
            Part::Name(_) | Part::Number(_) | Part::Node(_) => {
                if operation.is_none() {
                    left.push(buffer[idx].clone());
                } else {
                    right.push(buffer[idx].clone());
                }
            }
            Part::Call => {
                idx += 1;
                if idx < buffer.len() {
                    if let Part::Name(func) = buffer[idx].clone() {
                        operation = Some(Operation::Call(func));
                    } else {
                        panic!("Expected function name after call operator!");
                    }
                } else {
                    panic!("Unexpected end of tokens after call operator!");
                }
            }
        }
        idx += 1;
    }
    Expression {
        operation,
        left: parse_unaries(&left),
        right: parse_unaries(&right),
    }
}
fn parse_unaries(buffer: &Vec<Part>) -> Vec<Value> {
    buffer
        .iter()
        .map(|part| match part.clone() {
            Part::Name(s) => Value::Name(s),
            Part::Number(x) => Value::Number(x),
            Part::Node(n) => match n {
                AstNode::Expression(expr) => Value::Expression(expr),
                _ => panic!("Block found while parsing unary expression"),
            },
            _ => panic!("can't parse unary expression"),
        })
        .collect()
}
