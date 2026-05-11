use super::iteration::ir_iteration;
use super::value::ir_value;
use super::*;
use crate::parser::{Computation, Expression, Operation, Value};
use std::collections::HashMap;
pub(super) fn ir_expression(
    expression: &Expression,
    variables: &mut HashMap<String, usize>,
    index: usize,
    outer: Option<Operation>,
) -> Vec<Command> {
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
                Operation::Vector(_) => {
                    commands.append(&mut ir_iteration(expression, variables, index + 1, outer));
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
                                Some(op),
                            ));
                        }
                        commands.push(command.unwrap());
                    }
                    Computation::Sub => {
                        for (idx, value) in expression.left.iter().enumerate() {
                            commands.append(&mut ir_value(
                                value,
                                variables,
                                index + commands.len(),
                                Some(op),
                            ));
                            if idx > 0 {
                                commands.push(Command::Add);
                            }
                        }
                        for (idx, value) in expression.right.iter().enumerate() {
                            commands.append(&mut ir_value(
                                value,
                                variables,
                                index + commands.len(),
                                Some(op),
                            ));
                            if idx > 0 {
                                commands.push(Command::Add);
                            }
                        }
                        commands.push(command.unwrap());
                    }
                    Computation::Mul => {
                        commands.push(Command::Put(StackValue::Int(1)));
                        for value in expression.left.iter().chain(expression.right.iter()) {
                            commands.append(&mut ir_value(
                                value,
                                variables,
                                index + commands.len(),
                                Some(op),
                            ));
                        }
                        commands.push(command.unwrap());
                    }
                    Computation::Div => {
                        for (idx, value) in expression.left.iter().enumerate() {
                            commands.append(&mut ir_value(
                                value,
                                variables,
                                index + commands.len(),
                                Some(op),
                            ));
                            if idx > 0 {
                                commands.push(Command::Mul);
                            }
                        }

                        for (idx, value) in expression.right.iter().enumerate() {
                            commands.append(&mut ir_value(
                                value,
                                variables,
                                index + commands.len(),
                                Some(op),
                            ));
                            if idx > 0 {
                                commands.push(Command::Mul);
                            }
                        }
                        commands.push(command.unwrap());
                    }
                    Computation::Mod => {
                        if expression.left.len() + expression.right.len() == 2 {
                            for i in expression.left.iter().chain(expression.right.iter()) {
                                commands.append(&mut ir_value(
                                    i,
                                    variables,
                                    index + commands.len(),
                                    None,
                                ));
                            }
                        }
                        commands.push(command.unwrap());
                    }
                },
                Operation::Logic(_) => {
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
                            Some(op),
                        ));
                        if idx != 0 {
                            commands.push(command.clone().unwrap());
                        }
                    }
                }
            }
        }
        None => {
            if expression.left.len() == 1 {
                commands.append(&mut ir_value(
                    &expression.left[0],
                    variables,
                    index + commands.len(),
                    None,
                ));
            } else {
                panic!("Expression with no operation and more than one value found!");
            }
        }
    }
    commands
}
