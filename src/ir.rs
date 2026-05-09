use std::collections::HashMap;

use crate::parser::{AstNode, Comparison, Computation, Expression, Logic, Operation, Value};
#[derive(Debug, Clone, Copy)]
enum StackValue {
    Int(isize),
    Bool(bool),
}
#[derive(Debug, Clone, Copy)]
pub enum Command {
    //computation
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    // stack operations
    Put(StackValue),
    Pop,
    // comparision
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
}
pub fn ir(root: AstNode, variables: &mut HashMap<String, usize>) -> Vec<Command> {
    let mut commands = Vec::new();
    match root {
        AstNode::Expression(expression) => {
            commands.append(&mut ir_expression(&expression, variables))
        }
        AstNode::BlockVec(ast_nodes) => todo!(),
        AstNode::BlockCode(nodes) => {
            for node in nodes {
                commands.append(&mut ir(node, variables));
            }
        }
    }
    commands
}
fn ir_value(value: &Value, variables: &mut HashMap<String, usize>) -> Vec<Command> {
    let mut commands = Vec::new();
    match value {
        Value::Name(s) => {
            if variables.contains_key(s) {
                commands.push(Command::Load(variables[s]));
            } else {
                variables.insert(s.clone(), variables.len());
                commands.push(Command::Load(variables[s]));
            }
        }
        Value::Number(x) => commands.push(Command::Put(StackValue::Int(*x))),
        Value::Expression(expr) => {
            commands.append(&mut ir_expression(expr, variables));
        }
    }
    commands
}
fn ir_expression(expression: &Expression, variables: &mut HashMap<String, usize>) -> Vec<Command> {
    let mut commands = Vec::new();
    match expression.operation {
        Some(op) => {
            let command = operation_to_command(op);
            println!(
                "right len: {}; left len: {}; command: {:?}",
                expression.left.len(),
                expression.right.len(),
                command
            );
            match op {
                Operation::Comparison(_) => {
                    assert_eq!(expression.left.len(), expression.right.len());
                    for i in 0..expression.left.len() {
                        commands.append(&mut ir_value(&expression.left[i], variables));
                        commands.append(&mut ir_value(&expression.right[i], variables));
                        commands.push(command);
                    }
                }
                Operation::Computation(computation) => match computation {
                    Computation::Add => {
                        for (idx, value) in expression
                            .left
                            .iter()
                            .chain(expression.right.iter())
                            .enumerate()
                        {
                            commands.append(&mut ir_value(value, variables));
                            if idx != 0 {
                                commands.push(command);
                            }
                        }
                    }
                    Computation::Sub => {
                        for (idx, value) in expression.left.iter().enumerate() {
                            commands.append(&mut ir_value(value, variables));
                            if idx > 0 {
                                commands.push(Command::Add);
                            }
                        }
                        for (idx, value) in expression.right.iter().enumerate() {
                            commands.append(&mut ir_value(value, variables));
                            if idx > 0 {
                                commands.push(Command::Add);
                            }
                        }
                        commands.push(command);
                    }
                    Computation::Mul => {
                        for (idx, value) in expression.left.iter().enumerate() {
                            commands.append(&mut ir_value(value, variables));
                            if idx > 0 {
                                commands.push(command);
                            }
                        }
                        for (idx, value) in expression.right.iter().enumerate() {
                            commands.append(&mut ir_value(value, variables));
                            if idx + expression.left.len() > 0 {
                                commands.push(command);
                            }
                        }
                    }
                    Computation::Div => {
                        for (idx, value) in expression.left.iter().enumerate() {
                            commands.append(&mut ir_value(value, variables));
                            if idx > 0 {
                                commands.push(Command::Mul);
                            }
                        }
                        for (idx, value) in expression.right.iter().enumerate() {
                            commands.append(&mut ir_value(value, variables));
                            if idx > 0 {
                                commands.push(Command::Mul);
                            }
                        }
                        commands.push(command);
                    }
                    Computation::Mod => {
                        if expression.left.len() + expression.right.len() == 2 {
                            for i in expression.left.iter().chain(expression.right.iter()) {
                                commands.append(&mut ir_value(i, variables));
                            }
                        }
                        commands.push(command);
                    }
                },
                Operation::Logic(_) => {
                    for (idx, value) in expression
                        .left
                        .iter()
                        .chain(expression.right.iter())
                        .enumerate()
                    {
                        commands.append(&mut ir_value(value, variables));
                        if idx != 0 {
                            commands.push(command);
                        }
                    }
                }
            }
        }
        None => {
            if expression.left.len() == 1 {
                commands.append(&mut ir_value(&expression.left[0], variables));
            }
        }
    }
    commands
}
fn operation_to_command(op: Operation) -> Command {
    match op {
        Operation::Comparison(comparison) => match comparison {
            Comparison::Greater => Command::Gt,
            Comparison::Less => Command::Ls,
            Comparison::Equal => Command::Eq,
            Comparison::GreaterOrEqual => Command::Geq,
            Comparison::LessOrEqual => Command::Leq,
            Comparison::NotEqual => Command::Neq,
        },
        Operation::Computation(computation) => match computation {
            Computation::Add => Command::Add,
            Computation::Sub => Command::Sub,
            Computation::Mul => Command::Mul,
            Computation::Div => Command::Div,
            Computation::Mod => Command::Mod,
        },
        Operation::Logic(logic) => match logic {
            Logic::And => Command::And,
            Logic::Or => Command::Or,
            Logic::Xor => Command::Xor,
            Logic::Nand => Command::Nand,
            Logic::Nor => Command::Nor,
            Logic::Not => Command::Not,
        },
    }
}
