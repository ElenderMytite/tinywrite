use super::iteration::ir_iteration;
use super::value::ir_value;
use super::*;
use crate::parser::types::{Computation, Expression, Operation, Value};
use std::collections::HashMap;
pub(super) fn ir_expression(
    expression: &Expression,
    variables: &mut HashMap<String, usize>,
    index: usize,
    outer: Option<Operation>,
) -> Vec<Command> {
    let mut commands = Vec::new();
    match expression.operation.clone() {
        Some(op) => {
            let command = operation_to_command(op.clone());
            // println!(
            //     "right len: {}; left len: {}; command: {:?}",
            //     expression.left.len(),
            //     expression.right.len(),
            //     command
            // );
            match op {
                Operation::Comparison(_) | Operation::Computation(_) | Operation::Logic(_)
                    if expression.left.len() + expression.right.len() == 2 =>
                {
                    for value in expression.left.iter().chain(expression.right.iter()) {
                        commands.append(&mut ir_value(
                            value,
                            variables,
                            index + commands.len(),
                            None,
                        ));
                    }
                    commands.push(command.clone().unwrap());
                    return commands;
                }
                Operation::Vector(_) => {
                    commands.append(&mut ir_iteration(expression, variables, index + 1, outer));
                }
                Operation::Call(_) => {
                    let new_idx = index + commands.len();
                    call::ir_call(
                        expression,
                        variables,
                        &mut commands,
                        command,
                        new_idx,
                        outer,
                    );
                }
                Operation::Set => {
                    assert_eq!(expression.left.len(), expression.right.len());
                    for i in 0..expression.left.len() {
                        assert!(matches!(expression.left[i], Value::Name(_)));
                        commands.append(&mut ir_value(
                            &expression.right[i],
                            variables,
                            index + commands.len(),
                            None,
                        ));
                        commands.push(Command::Store(register_variable(
                            variables,
                            expression.left[i].get_name().unwrap(),
                        )))
                    }
                }
                Operation::Comparison(_) => {
                    assert_eq!(expression.left.len(), expression.right.len());
                    for i in 0..expression.left.len() {
                        commands.append(&mut ir_value(
                            &expression.left[i],
                            variables,
                            index + commands.len(),
                            None,
                        ));
                        commands.append(&mut ir_value(
                            &expression.right[i],
                            variables,
                            index + commands.len(),
                            None,
                        ));
                        commands.push(command.clone().unwrap());
                    }
                }
                Operation::Computation(computation) => match computation {
                    Computation::Add => {
                        commands.push(Command::Put(StackValue::Int(0)));
                        for value in expression.left.iter().chain(expression.right.iter()) {
                            commands.append(&mut ir_value(
                                value,
                                variables,
                                index + commands.len(),
                                Some(op.clone()),
                            ));
                            commands.push(Command::Add);
                        }
                    }
                    Computation::Sub => {
                        commands.push(Command::Put(StackValue::Int(0)));
                        for value in expression.left.iter() {
                            commands.append(&mut ir_value(
                                value,
                                variables,
                                index + commands.len(),
                                Some(op.clone()),
                            ));
                            commands.push(Command::Add);
                        }
                        commands.push(Command::Put(StackValue::Int(0)));
                        for value in expression.right.iter() {
                            commands.append(&mut ir_value(
                                value,
                                variables,
                                index + commands.len(),
                                Some(op.clone()),
                            ));
                            commands.push(Command::Add);
                        }
                        commands.push(Command::Sub);
                    }
                    Computation::Mul => {
                        commands.push(Command::Put(StackValue::Int(1)));
                        for value in expression.left.iter().chain(expression.right.iter()) {
                            commands.append(&mut ir_value(
                                value,
                                variables,
                                index + commands.len(),
                                Some(op.clone()),
                            ));
                            commands.push(Command::Mul);
                        }
                    }
                    Computation::Div => {
                        commands.push(Command::Put(StackValue::Int(1)));
                        for value in expression.left.iter() {
                            commands.append(&mut ir_value(
                                value,
                                variables,
                                index + commands.len(),
                                Some(op.clone()),
                            ));
                            commands.push(Command::Mul);
                        }
                        commands.push(Command::Put(StackValue::Int(1)));
                        for value in expression.right.iter() {
                            commands.append(&mut ir_value(
                                value,
                                variables,
                                index + commands.len(),
                                Some(op.clone()),
                            ));
                            commands.push(Command::Mul);
                        }
                        commands.push(Command::Div);
                    }
                    Computation::Mod => {
                        panic!(
                            "Mod operation is only supported for 2 arguments; {} provided.",
                            expression.left.len() + expression.right.len()
                        );
                    }
                },
                Operation::Logic(l) => {
                    for (idx, value) in expression
                        .left
                        .iter()
                        .chain(expression.right.iter())
                        .enumerate()
                    {
                        commands.append(&mut ir_value(
                            value,
                            variables,
                            index + commands.len(),
                            Some(op.clone()),
                        ));
                        if idx != 0 || l == Logic::Not {
                            commands.push(command.clone().unwrap());
                        }
                    }
                }
            }
        }
        None => {
            for i in 0..expression.left.len() {
                commands.append(&mut ir_value(
                    &expression.left[i],
                    variables,
                    index + commands.len(),
                    None,
                ));
            }
        }
    }
    commands
}
