use std::collections::HashMap;
mod call;
mod expression;
mod iteration;
mod value;
use crate::TranslationError::UnexpectedValue;
use crate::ir::TranslationError::UnknownIdentifier;
use crate::parser;
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
type Commands = Vec<Command>;
pub fn translate(
    root: AstNode,
    variables: &mut HashMap<String, usize>,
) -> Result<Commands, TranslationError> {
    let mut commands = Commands::new();
    ir(root, variables, &mut commands)?;
    commands.push(Command::Call(0));
    Ok(commands)
}
fn ir(
    root: AstNode,
    variables: &mut HashMap<String, usize>,
    commands: &mut Vec<Command>,
) -> Result<(), TranslationError> {
    match root {
        AstNode::Expression(expr) => {
            expression::ir_expression(&expr, variables, None, commands)?;
        }
        AstNode::BlockCode(nodes) => {
            for node in nodes {
                ir(node, variables, commands)?;
            }
        }
    }
    //dbg!(&commands);
    Ok(())
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
#[derive(Debug, Clone)]
pub enum TranslationError {
    UnknownIdentifier(String),
    UnexpectedValue(parser::types::Value),
    UncovertableOperation,
}
impl From<parser::types::Value> for TranslationError {
    fn from(value: parser::types::Value) -> Self {
        UnexpectedValue(value)
    }
}
impl From<String> for TranslationError {
    fn from(value: String) -> Self {
        UnknownIdentifier(value)
    }
}
impl From<Operation> for Result<Command, TranslationError> {
    fn from(value: Operation) -> Self {
        match value {
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
            Operation::Call(func) => Ok(match func.as_str() {
                "push" => Command::VPush,
                "pop" => Command::VPop,
                "get" => Command::Get,
                "len" => Command::Len,
                "print" => {
                    dbg!("converting to print command");
                    Command::Call(1)
                }
                "in" => Command::HContains,
                "add" => Command::HInsert,
                "remove" => Command::HRemove,
                "hmap" => Command::HNew,
                "vec" => Command::VNew,
                _ => return Err(TranslationError::UncovertableOperation),
            }),
            _ => Err(TranslationError::UncovertableOperation),
        }
    }
}
