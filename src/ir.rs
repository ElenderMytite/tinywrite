use std::collections::HashMap;
mod call;
mod expression;
mod iteration;
mod value;
use crate::ParseError;
use crate::parser::types::{AstNode, Comparison, Computation, Logic, Operation};
use crate::vm::StackValue;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Command {
    //computation
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    // stack operations
    Dup,
    Del,
    Swap,
    Put(StackValue),
    // comparison
    Gt,
    Geq,
    Ls,
    Leq,
    Eq,
    Neq,
    // logic
    Not,
    And,
    Or,
    Xor,
    Nor,
    Nand,
    // variable operations
    Load(usize),
    Store(usize),
    // control flow,
    Jmp(usize),
    // vector operations,
    Len,
    VNew,
    VPush,
    VPop,
    Get,
    // hashmap operations,
    HNew,
    HInsert,
    HContains,
    HRemove,
    // Group(pair) operations,
    Group,
    Prepend,
    // type conversions
    Byte, // convert ascii character to int
    Char, // convert int to ascii character
    // I/O
    Call(usize),
}
pub fn translate(
    root: AstNode,
    variables: &mut HashMap<String, usize>,
) -> Result<Vec<Command>, ParseError> {
    let mut commands = ir(root, variables, 0)?;
    commands.push(Command::Call(0));
    Ok(commands)
}
fn ir(
    root: AstNode,
    variables: &mut HashMap<String, usize>,
    /* index of the first command of this function for correct jumps*/ index: usize,
) -> Result<Vec<Command>, ParseError> {
    let mut commands = Vec::new();
    match root {
        AstNode::Expression(expression) => {
            commands.append(&mut expression::ir_expression(
                &expression,
                variables,
                index + commands.len(),
                None,
            )?);
        }
        AstNode::BlockCode(nodes) => {
            for node in nodes {
                commands.append(&mut ir(node, variables, index + commands.len())?);
            }
        }
    }
    //dbg!(&commands);
    Ok(commands)
}
/// Registers a variable in the environment and returns its index. If the variable already exists, it just returns the existing index.
fn register_variable(env: &mut HashMap<String, usize>, variable: String) -> usize {
    if !env.contains_key(&variable) {
        env.insert(variable.clone(), env.len());
    }
    env[&variable]
}
fn free_variable(env: &mut HashMap<String, usize>, variable: String) {
    env.remove(&variable);
}
fn operation_to_command(op: Operation) -> Result<Command, String> {
    match op {
        Operation::Comparison(comparison) => Ok(match comparison {
            Comparison::Greater => Command::Gt,
            Comparison::Less => Command::Ls,
            Comparison::Equal => Command::Eq,
            Comparison::GreaterOrEqual => Command::Geq,
            Comparison::LessOrEqual => Command::Leq,
            Comparison::NotEqual => Command::Neq,
        }),
        Operation::Computation(computation) => Ok(match computation {
            Computation::Add => Command::Add,
            Computation::Sub => Command::Sub,
            Computation::Mul => Command::Mul,
            Computation::Div => Command::Div,
            Computation::Mod => Command::Mod,
        }),
        Operation::Logic(logic) => Ok(match logic {
            Logic::And => Command::And,
            Logic::Or => Command::Or,
            Logic::Xor => Command::Xor,
            Logic::Nand => Command::Nand,
            Logic::Nor => Command::Nor,
            Logic::Not => Command::Not,
        }),
        Operation::Call(func) => match func.as_str() {
            "push" => Ok(Command::VPush),
            "pop" => Ok(Command::VPop),
            "get" => Ok(Command::Get),
            "len" => Ok(Command::Len),
            "print" => Ok(Command::Call(1)),
            "in" => Ok(Command::HContains),
            "add" => Ok(Command::HInsert),
            "remove" => Ok(Command::HRemove),
            "hmap" => Ok(Command::HNew),
            "vec" => Ok(Command::VNew),
            _ => Err(format!("unknown function: {}", func)),
        },
        _ => Err("Unsupported operation in ir generation!".to_string()),
    }
}
