use std::collections::HashMap;

use crate::{
    ParseError,
    ir::{Command, iteration::ir_vector_operation, register_variable, value::ir_value},
    parser::types::{Expression, Operation},
};
pub(super) fn ir_call(
    expression: &Expression,
    variables: &mut HashMap<String, usize>,
    commands: &mut Vec<Command>,
    command: Result<Command, String>,
    index: usize,
    outer: Option<Operation>,
) -> Result<(), ParseError> {
    let func = match &expression.operation {
        Some(Operation::Call(func)) => func.clone(),
        _ => panic!("Non-call expression found in ir_call!"),
    };
    match func.as_str() {
        "byte" | "char" => {
            assert_eq!(expression.left.len() + expression.right.len(), 1);
            let value = if expression.left.len() == 1 {
                &expression.left[0]
            } else {
                &expression.right[0]
            };
            commands.append(&mut ir_value(
                value,
                variables,
                index + commands.len(),
                None,
            )?);
            match func.as_str() {
                "byte" => commands.push(Command::Byte),
                "char" => commands.push(Command::Char),
                _ => unreachable!(),
            }
            return Ok(());
        }
        "push" | "get" | "add" | "remove" | "in" => {
            assert_eq!(expression.left.len(), 1);
            commands.push(Command::Load(register_variable(
                variables,
                expression.left[0].get_name()?,
            )));
        }
        "len" | "pop" => {
            assert_eq!(expression.left.len() + expression.right.len(), 1);
            if expression.left.len() == 1 {
                commands.push(Command::Load(register_variable(
                    variables,
                    expression.left[0].get_name().unwrap(),
                )));
            } else {
                commands.push(Command::Load(register_variable(
                    variables,
                    expression.right[0].get_name().unwrap(),
                )));
            }
            commands.push(command.clone().unwrap());
            return Ok(());
        }
        "hmap" => {
            commands.push(command.clone().unwrap());
            return Ok(());
        }
        "vec" => commands.append(&mut ir_vector_operation(
            expression, variables, index, outer,
        )?),
        "print" => (),
        _ => {
            panic!("Unsupported function call found!");
        }
    }
    for value in expression
        .left
        .iter()
        .skip(1)
        .chain(expression.right.iter())
    {
        // special prepraration for commands that consume the pointer to the vector (get command),
        match command {
            Ok(Command::Get) => commands.push(Command::Dup),
            _ => (),
        }
        commands.append(&mut ir_value(
            value,
            variables,
            index + commands.len(),
            None,
        )?);
        match command {
            Ok(Command::HInsert) => (),
            Ok(_) => {
                commands.push(command.clone().unwrap());
            }
            _ => panic!(
                "Unsupported function call found! Command {:?} not supported;",
                command
            ),
        }
        // special post-preparation for commands that put values on top of the stack (get command),
        match command {
            Ok(Command::Get) => commands.push(Command::Swap),
            _ => (),
        }
    }
    match command {
        Ok(Command::Get) => commands.push(Command::Del),
        Ok(Command::HInsert) => commands.push(command.clone().unwrap()),
        _ => (),
    }
    Ok(())
}
