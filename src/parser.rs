pub mod types;
use std::fmt::Display;

use types::{
    AstNode, Comparison, Computation, Constant, Folder, Logic, Operation, ParsingMode, Part, Value,
    VectorOp,
};

use crate::{
    lexer::Token::{self},
    parser::types::{BinaryExpression, ExpressionNode},
};
#[derive(Debug, Clone)]
pub enum ParseError {
    UnexpectedConstant(String),
    UnexpectedEOFAfter(Token),
    UnexpectedToken(Token),
    UnexpectedEndOfExpression,
    UnexpectedBlock,
    NameError(Value),
}

impl Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let msg = match self {
            ParseError::UnexpectedConstant(constant) => {
                format!("Unexpected constant: {}", constant)
            }
            ParseError::UnexpectedEOFAfter(token) => {
                format!("Unexpected end of file(EOF) after '{:?}' token", token)
            }
            ParseError::UnexpectedToken(token) => {
                format!("Unexpected token: {:?}", token)
            }
            ParseError::UnexpectedEndOfExpression => {
                format!("Unexpected end of expression")
            }
            ParseError::NameError(value) => {
                format!("{value:?} is not a name")
            }
            ParseError::UnexpectedBlock => String::from("Unexpected block!"),
        };
        write!(f, "{}", msg)
    }
}
pub fn astify(
    tokens: &Vec<Token>,
    block_type: ParsingMode,
    index: &mut usize,
) -> Result<AstNode, ParseError> {
    let mut buffer: Vec<Part> = Vec::new();
    let mut nodes: Vec<AstNode> = Vec::new();
    while *index < tokens.len() {
        match tokens[*index].clone() {
            Token::RoundOpen => {
                *index += 1;
                buffer.push(Part::Value(Value::Expression(
                    astify(tokens, ParsingMode::Expression, index)?.expr()?,
                )));
            }
            Token::CurlyOpen => {
                *index += 1;
                nodes.push(astify(tokens, ParsingMode::Code, index).unwrap())
            }
            Token::RoundClose => {
                if block_type == ParsingMode::Expression {
                    let expr = parse_expression(&buffer);
                    buffer.clear();
                    return Ok(AstNode::Expression(expr));
                }
            }
            Token::CurlyClosed => {
                if block_type == ParsingMode::Code {
                    return Ok(AstNode::BlockCode(nodes));
                }
            }
            Token::EndOfStatement => match block_type {
                ParsingMode::Expression => {
                    let expr = Part::Value(Value::Expression(parse_expression(&buffer)));
                    buffer.clear();
                    buffer.push(expr.clone());
                }
                ParsingMode::Code => {
                    if !buffer.is_empty() {
                        let expr = parse_expression(&buffer);
                        buffer.clear();
                        nodes.push(AstNode::Expression(expr));
                    }
                }
            },
            Token::Assign => buffer.push(Part::MultiOperation(Operation::Set)),
            Token::Call => buffer.push(Part::Call),
            Token::Plus => buffer.push(Part::Operation(Operation::Computation(Computation::Add))),
            Token::Minus => buffer.push(Part::Operation(Operation::Computation(Computation::Sub))),
            Token::Asterisk => {
                buffer.push(Part::Operation(Operation::Computation(Computation::Mul)))
            }
            Token::Slash => buffer.push(Part::Operation(Operation::Computation(Computation::Div))),
            Token::Modulo => buffer.push(Part::Operation(Operation::Computation(Computation::Mod))),
            Token::And => buffer.push(Part::Operation(Operation::Logic(Logic::Or))),
            Token::Or => buffer.push(Part::Operation(Operation::Logic(Logic::And))),
            Token::Xor => buffer.push(Part::Operation(Operation::Logic(Logic::Xor))),
            Token::Nor => buffer.push(Part::Operation(Operation::Logic(Logic::Nor))),
            Token::Nand => buffer.push(Part::Operation(Operation::Logic(Logic::Nand))),
            Token::Negation => buffer.push(Part::Operation(Operation::Logic(Logic::Not))),
            Token::Equality => {
                buffer.push(Part::Operation(Operation::Comparison(Comparison::Equal)))
            }
            Token::Unequality => {
                buffer.push(Part::Operation(Operation::Comparison(Comparison::NotEqual)))
            }
            Token::Multinequality => {
                buffer.push(Part::MultiOperation(Operation::Comparison(
                    Comparison::Equal,
                )));
            }
            Token::StrictMore => {
                buffer.push(Part::Operation(Operation::Comparison(Comparison::Greater)))
            }
            Token::StrictLess => {
                buffer.push(Part::Operation(Operation::Comparison(Comparison::Less)))
            }
            Token::LooseMore => buffer.push(Part::Operation(Operation::Comparison(
                Comparison::GreaterOrEqual,
            ))),
            Token::LooseLess => buffer.push(Part::Operation(Operation::Comparison(
                Comparison::LessOrEqual,
            ))),
            Token::Glue => buffer.push(Part::Operation(Operation::Vector(VectorOp::Pack))),
            Token::Slice => buffer.push(Part::Operation(Operation::Vector(VectorOp::Unpack))),

            Token::Int(x) => {
                buffer.push(Part::Value(Value::Number(x)));
            }
            Token::Name(name) => {
                buffer.push(Part::Value(Value::Name(name)));
            }
            Token::String(s) => buffer.push(Part::Value(Value::Literal(s))),
            Token::Constant => {
                *index += 1;
                if let Some(Token::Name(keyword)) = tokens.get(*index) {
                    match keyword.as_str() {
                        "true" => buffer.push(Part::Constant(Constant::True)),
                        "false" => buffer.push(Part::Constant(Constant::False)),
                        "tab" => buffer.push(Part::Constant(Constant::Tab)),
                        "line" => buffer.push(Part::Constant(Constant::Newline)),
                        "space" => buffer.push(Part::Constant(Constant::Space)),
                        // only works for ascii
                        c if keyword.len() == 1 => {
                            buffer.push(Part::Value(Value::Char(c.chars().nth(0).unwrap())))
                        }
                        func => return Err(ParseError::UnexpectedConstant(func.to_string())),
                    }
                } else {
                    return Err(ParseError::UnexpectedEOFAfter(Token::Constant));
                }
            }
            token => return Err(ParseError::UnexpectedToken(token.clone())),
        }
        *index += 1;
    }
    parse_expression(&buffer);
    //dbg!(&nodes);
    match block_type {
        ParsingMode::Expression => return Err(ParseError::UnexpectedEndOfExpression),
        ParsingMode::Code => Ok(AstNode::BlockCode(nodes)),
    }
}
fn parse_expression(buffer: &Vec<Part>) -> ExpressionNode {
    let mut idx = 0;
    let mut multi: bool = false;
    let mut left: Vec<Part> = Vec::new();
    let mut right: Vec<Part> = Vec::new();
    let mut operation: Option<Operation> = None;
    while idx < buffer.len() {
        match buffer[idx].clone() {
            Part::MultiOperation(multiop) => {
                if operation.is_none() {
                    operation = Some(multiop);
                    multi = true;
                } else if operation != Some(multiop) {
                    panic!("two kinds of operations found inside one expression")
                }
            }
            Part::Operation(op) => {
                if operation.is_none() {
                    operation = Some(op);
                    multi = false;
                } else if operation != Some(op) {
                    panic!("two kinds of operations found inside one expression")
                }
            }
            Part::Value(_) => {
                if operation.is_none() {
                    left.push(buffer[idx].clone());
                } else {
                    right.push(buffer[idx].clone());
                }
            }
            Part::Call => {
                multi = true;
                idx += 1;
                if idx < buffer.len() {
                    if let Part::Value(Value::Name(func)) = buffer[idx].clone() {
                        operation = Some(Operation::Call(func));
                    } else {
                        panic!("Expected function name after call operator!");
                    }
                } else {
                    panic!("Unexpected end of tokens after call operator!");
                }
            }
            Part::Constant(_) => {
                if operation.is_none() {
                    left.push(buffer[idx].clone());
                } else {
                    right.push(buffer[idx].clone());
                }
            }
        }
        idx += 1;
    }
    if multi {
        ExpressionNode::Multi(Box::new(Folder {
            operation,
            left: parse_unaries(&left),
            right: parse_unaries(&right),
        }))
    } else if let (None, true) = (operation.clone(), right.is_empty()) {
        ExpressionNode::Multi(Box::new(Folder {
            operation,
            left: parse_unaries(&left),
            right: vec![],
        }))
    } else {
        assert_eq!(left.len() + right.len(), 2);
        let mut args = left.iter().chain(right.iter());
        let left = parse_unary(args.next().unwrap()).clone();
        let right = parse_unary(args.next().unwrap()).clone();
        ExpressionNode::Binary(Box::new(BinaryExpression {
            operation,
            left,
            right,
        }))
    }
    //dbg!(&expr);
}
fn parse_unary(part: &Part) -> &Value {
    match part {
        Part::Value(x) => x,
        Part::Constant(b) => match b {
            Constant::True => &Value::Bool(true),
            Constant::False => &Value::Bool(false),
            Constant::Tab => &Value::Char('\t'),
            Constant::Newline => &Value::Char('\n'),
            Constant::Space => &Value::Char(' '),
        },
        _ => panic!("can't parse unary expression"),
    }
}
fn parse_unaries(buffer: &Vec<Part>) -> Vec<Value> {
    buffer
        .iter()
        .map(|part| parse_unary(part))
        .cloned()
        .collect()
}
