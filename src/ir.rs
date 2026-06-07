use std::collections::HashMap;
mod expression;
mod iteration;
mod value;
use crate::parser::types::{AstNode, Comparison, Computation, Logic, Operation};
use crate::vm::StackValue;
#[derive(Debug, Clone)]
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
    Cls,
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
    Push,
    VPop,
    Get,
    // type conversions
    Byte, // convert ascii character to int
    Char, // convert int to ascii character
}
pub fn ir(
    root: AstNode,
    variables: &mut HashMap<String, usize>,
    /* index of the first command of this function for correct jumps*/ index: usize,
) -> Vec<Command> {
    let mut commands = Vec::new();
    match root {
        AstNode::Expression(expression) => {
            commands.append(&mut expression::ir_expression(
                &expression,
                variables,
                index + commands.len(),
                None,
            ));
        }
        AstNode::BlockCode(nodes) => {
            for node in nodes {
                commands.append(&mut ir(node, variables, index + commands.len()));
                commands.push(Command::Cls);
            }
        }
    }
    //dbg!(&commands);
    commands
}

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
            "push" => Ok(Command::Push),
            "pop" => Ok(Command::VPop),
            "get" => Ok(Command::Get),
            "len" => Ok(Command::Len),
            _ => Err("unknown function".to_string()),
        },
        _ => Err("Unsupported operation in ir generation!".to_string()),
    }
}
