pub mod types;
use types::{
    AstNode, Comparison, Computation, Expression, Keyword, Logic, Operation, ParsingMode, Part,
    Value, VectorOp,
};
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
                buffer.push(Part::Expression(
                    astify(tokens, ParsingMode::Expression, index)
                        .unwrap()
                        .expr()
                        .unwrap(),
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
                    let expr = Part::Expression(parse_expression(&buffer));
                    buffer.clear();
                    buffer.push(expr.clone());
                }
                ParsingMode::Code => {
                    if !buffer.is_empty() {
                        let node = parse_expression(&buffer);
                        buffer.clear();
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
            "$" => {
                *index += 1;
                if let Some(keyword) = tokens.get(*index) {
                    match keyword.to_lowercase().as_str() {
                        "if" => buffer.push(Part::Keyword(Keyword::If)),
                        "else" => buffer.push(Part::Keyword(Keyword::Else)),
                        "end" => buffer.push(Part::Keyword(Keyword::End)),
                        "redo" => buffer.push(Part::Keyword(Keyword::Redo)),
                        "true" => buffer.push(Part::Keyword(Keyword::True)),
                        "false" => buffer.push(Part::Keyword(Keyword::False)),
                        _ => return Err(format!("unexpected keywrord: {keyword}")),
                    }
                } else {
                    return Err("expected keyword after $ sign, found end of file".to_string());
                }
            }
            x => return Err(format!("unexpected token: \"{x}\"")),
        }
        *index += 1;
    }
    if !buffer.is_empty() {
        for part in buffer {
            match part {
                Part::Expression(n) => nodes.push(AstNode::Expression(n)),
                _ => {
                    return Err(format!(
                        "unexpected part in buffer at end of tokens: {part:?}"
                    ));
                }
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
                } else if operation != Some(op) {
                    panic!("two kinds of operations found inside one expression")
                }
            }
            Part::Name(_) | Part::Number(_) | Part::Expression(_) => {
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
            Part::Keyword(word) => match word {
                Keyword::If | Keyword::Else | Keyword::End | Keyword::Redo => {
                    panic!("Unexpected control flow keyword inside expression!");
                }
                Keyword::True | Keyword::False => {
                    if operation.is_none() {
                        left.push(buffer[idx].clone());
                    } else {
                        right.push(buffer[idx].clone());
                    }
                }
            },
        }
        idx += 1;
    }
    let expr = Expression {
        operation,
        left: parse_unaries(&left),
        right: parse_unaries(&right),
    };
    //dbg!(&expr);
    expr
}
fn parse_unaries(buffer: &Vec<Part>) -> Vec<Value> {
    buffer
        .iter()
        .map(|part| match part.clone() {
            Part::Name(s) => Value::Name(s),
            Part::Number(x) => Value::Number(x),
            Part::Expression(expr) => Value::Expression(expr),
            Part::Keyword(b) => match b {
                Keyword::True => Value::Bool(true),
                Keyword::False => Value::Bool(false),
                _ => panic!("can't convert keyword to value"),
            },
            _ => panic!("can't parse unary expression"),
        })
        .collect()
}
