use std::rc::Rc;
#[derive(PartialEq, Eq)]
pub enum ParsingMode {
    Expression,
    BlockCode,
    BlockVec,
}
#[derive(Debug, Clone)]
pub enum AstNode {
    Expression(Operation),
    BlockVec(Vec<AstNode>),
    BlockCode(Vec<AstNode>),
}
#[derive(Debug, Clone)]
pub enum Operation {
    Comparison(Comparison, Rc<Option<Operation>>,Rc<Option<Operation>>),
    Computation(Computation, Rc<Option<Operation>>, Rc<Option<Operation>>),
    Logic(Logic, Rc<Option<Operation>>, Rc<Option<Operation>>),
    Name(String),
    Number(isize),
}
#[derive(Debug, Clone)]
enum Part {
    Comparison(Comparison),
    Computation(Computation),
    Logic(Logic),
    Node(AstNode),
    Name(String),
    Number(isize),
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
pub fn astify(tokens: &Vec<String>, block_type: ParsingMode, index: &mut usize) -> Result<AstNode, String> {
    let mut buffer: Vec<Part> = Vec::new();
    let mut nodes: Vec<AstNode> = Vec::new();
    while *index < tokens.len() {
        match tokens[*index].as_str() {
            "(" => {
                *index += 1;
                buffer.push(Part::Node(astify(tokens, ParsingMode::Expression, index).unwrap()));
            }
            "{" => {
                *index += 1;
                nodes.push(astify(tokens, ParsingMode::BlockCode, index).unwrap())
            }
            "[" => {
                *index += 1;
                nodes.push(astify(tokens, ParsingMode::BlockVec, index).unwrap())
            }
            ")" => {
                if block_type == ParsingMode::Expression {
                    let node = parse_expression(&buffer);
                    // println!("parsed expression: {:?}", node);
                    return Ok(node);
                }
            }
            "}" => {
                if block_type == ParsingMode::BlockCode {
                    return Ok(AstNode::BlockCode(nodes));
                }
            }
            "]" => {
                if block_type == ParsingMode::BlockVec {
                    return Ok(AstNode::BlockVec(nodes));
                }
            }
            "+" => {
                buffer.push(Part::Computation(Computation::Add));    
            }
            "-" => {
                buffer.push(Part::Computation(Computation::Sub));    
            }
            "*" => {
                buffer.push(Part::Computation(Computation::Mul));
            }
            "/" => {
                buffer.push(Part::Computation(Computation::Div));
            }
            "%" => {
                buffer.push(Part::Computation(Computation::Mod));
            }
            "|" => {
                buffer.push(Part::Logic(Logic::Or));
            }
            "&" => {
                buffer.push(Part::Logic(Logic::And));
            }
            "^" => {
                buffer.push(Part::Logic(Logic::Xor));
            }
            "!" => {
                buffer.push(Part::Logic(Logic::Not));
            }
            "==" | "!^" => {
                buffer.push(Part::Comparison(Comparison::Equal));
            }
            "!=" => {
                buffer.push(Part::Comparison(Comparison::NotEqual));
            }
            ">" => {
                buffer.push(Part::Comparison(Comparison::Greater));
            }
            "<" => {
                buffer.push(Part::Comparison(Comparison::Less));
            }
            ">=" | "=>" | "!<" | "<!" => {
                buffer.push(Part::Comparison(Comparison::GreaterOrEqual));
            }
            "<=" | "=<" | "!>" | ">!" => {
                buffer.push(Part::Comparison(Comparison::LessOrEqual));
            }
            "!&" => {
                buffer.push(Part::Logic(Logic::Nand));
            }
            "!|" => {
                buffer.push(Part::Logic(Logic::Nor));
            }
            x if x.chars().all(|c| c.is_numeric()) | 
            (x.starts_with("-") && x[1..].chars().all(|c| c.is_numeric())) =>
            {
                buffer.push(Part::Number(x.parse::<isize>().unwrap()));
            }
            x if x.chars().all(|c| c.is_alphanumeric() || c == '_') => {
                buffer.push(Part::Name(x.to_string()));
            }

            x => panic!("unexpected token: \"{x}\"")
        }
        // println!("buffer: {:?}", buffer);
        // println!("index: {}", *index);
        *index += 1;
    }
    if !buffer.is_empty() {
        for part in buffer {
            match part {
                Part::Node(n) => nodes.push(n),
                _ => panic!("unexpected part in buffer at end of tokens: {:?}", part)
            }
        }
    }
    match block_type {
        ParsingMode::Expression => panic!("unexpected end of expression"),
        ParsingMode::BlockCode => Ok(AstNode::BlockCode(nodes)),
        ParsingMode::BlockVec => Ok(AstNode::BlockVec(nodes)),
    }
}
fn parse_expression(buffer: &Vec<Part>) -> AstNode {
    let mut idx = 0;
    // println!("parsing expression from: {:?}", buffer);
    let mut left: Option<Part> = None;
    let mut right: Option<Part> = None;
    let mut operation: Option<Part> = None;
    while idx < buffer.len() {
        match buffer[idx] {
            Part::Comparison(_) | Part::Logic(_) | Part::Computation(_) => {
                if operation.is_none() {
                    operation = Some(buffer[idx].clone());
                }
            }
            Part::Name(_) | Part::Number(_) | Part::Node(_)=> {
                if left.is_none() {
                    left = Some(buffer[idx].clone());
                } else if right.is_none() {
                    right = Some(buffer[idx].clone());
                }
            }
        }
        idx += 1;
    }
    match operation {
    Some(x) => {
        let left_node: Option<Operation> = get_value_from_part(left);
        let right_node = get_value_from_part(right);
        match x {
            Part::Comparison(c) => AstNode::Expression(Operation::Comparison(c, 
                    Rc::new(left_node), Rc::new(right_node))),
            Part::Computation(c) => AstNode::Expression(Operation::Computation(c, 
                    Rc::new(left_node), Rc::new(right_node))),
            Part::Logic(l) => AstNode::Expression(Operation::Logic(l, 
                    Rc::new(left_node), Rc::new(right_node))),
            _ => panic!("unexpected operation: {:?}", x)
        }
    }
    None => {
        match left {
            Some(x) => {
                match x {
                    Part::Name(n) => AstNode::Expression(Operation::Name(n)),
                    Part::Number(n) => AstNode::Expression(Operation::Number(n)),
                    Part::Node(n) => n,
                    _ => panic!("unexpected part: {:?}", x)
                }
            }
            None => panic!("no left operand found")
            
        }
    }
}
}
fn get_value_from_part(part: Option<Part>) -> Option<Operation> {
    part.map(|pa| match pa {
            Part::Name(n) => Operation::Name(n),
            Part::Number(n) => Operation::Number(n),
            Part::Node(n) => match n {
                AstNode::Expression(op) => op,
                _ => panic!("unexpected node type: {:?}", n)
            },
            _ => panic!("unexpected part: {:?}", pa)
        })

}
