use super::iteration::ir_vector_operation;
use super::value::ir_value;
use super::*;
use crate::parser::types::{Computation, Expression, Operation, Value};
use std::collections::HashMap;
pub(super) fn ir_expression(
    expression: &Expression,
    variables: &mut HashMap<String, usize>,
    outer: Option<Operation>,
    commands: &mut Commands,
    strings: &mut Vec<String>,
) -> Result<(), TranslationError> {
    match expression.operation.clone() {
        Some(op) => {
            let command = Command::try_from(op.clone());
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
                        ir_value(value, variables, None, commands, strings)?;
                    }
                    commands.push(command.clone().unwrap());
                    return Ok(());
                }
                Operation::Vector(_) => {
                    ir_vector_operation(expression, variables, outer.clone(), commands, strings)?;
                }
                Operation::Call(_) => {
                    call::ir_call(expression, variables, commands, strings, outer)?;
                }
                Operation::Set => {
                    assert_eq!(expression.left.len(), expression.right.len());
                    for i in 0..expression.right.len() {
                        assert!(matches!(expression.left[i], Value::Name(_)));
                        ir_value(
                            &expression.right[i],
                            variables,
                            outer.clone(),
                            commands,
                            strings,
                        )?;
                    }
                    for i in (0..expression.left.len()).rev() {
                        commands.push(Command::Store(register_variable(
                            variables,
                            (expression.left[i].get_name())?,
                        )))
                    }
                }
                Operation::Comparison(_) => {
                    assert_eq!(expression.left.len(), expression.right.len());
                    for i in 0..expression.left.len() {
                        ir_value(
                            &expression.left[i],
                            variables,
                            outer.clone(),
                            commands,
                            strings,
                        )?;
                        ir_value(
                            &expression.right[i],
                            variables,
                            outer.clone(),
                            commands,
                            strings,
                        )?;
                        commands.push(command.clone().unwrap());
                    }
                }
                Operation::Computation(computation) => match computation {
                    Computation::Add => {
                        commands.push(Command::Put(PrimitiveValue::Int(0)));
                        for value in expression.left.iter().chain(expression.right.iter()) {
                            ir_value(value, variables, outer.clone(), commands, strings)?;
                            commands.push(Command::Add);
                        }
                    }
                    Computation::Sub => {
                        commands.push(Command::Put(PrimitiveValue::Int(0)));
                        for value in expression.left.iter() {
                            ir_value(value, variables, outer.clone(), commands, strings)?;
                            commands.push(Command::Add);
                        }
                        commands.push(Command::Put(PrimitiveValue::Int(0)));
                        for value in expression.right.iter() {
                            ir_value(value, variables, outer.clone(), commands, strings)?;
                            commands.push(Command::Add);
                        }
                        commands.push(Command::Sub);
                    }
                    Computation::Mul => {
                        commands.push(Command::Put(PrimitiveValue::Int(1)));
                        for value in expression.left.iter().chain(expression.right.iter()) {
                            ir_value(value, variables, outer.clone(), commands, strings)?;
                            commands.push(Command::Mul);
                        }
                    }
                    Computation::Div => {
                        commands.push(Command::Put(PrimitiveValue::Int(1)));
                        for value in expression.left.iter() {
                            ir_value(value, variables, outer.clone(), commands, strings)?;
                            commands.push(Command::Mul);
                        }
                        commands.push(Command::Put(PrimitiveValue::Int(1)));
                        for value in expression.right.iter() {
                            ir_value(value, variables, outer.clone(), commands, strings)?;
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
                        ir_value(value, variables, Some(op.clone()), commands, strings)?;
                        if idx != 0 || l == Logic::Not {
                            commands.push(command.clone().unwrap());
                        }
                    }
                }
            }
        }
        None => {
            for i in 0..expression.left.len() {
                ir_value(&expression.left[i], variables, None, commands, strings)?;
            }
        }
    }
    Ok(())
}
