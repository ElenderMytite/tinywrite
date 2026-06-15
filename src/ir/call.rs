use std::collections::HashMap;

use crate::{
    ir::{
        Command, TranslationError, iteration::ir_vector_operation, register_variable,
        value::ir_value,
    },
    parser::types::{Expression, Operation},
};
pub(super) fn ir_call(
    expression: &Expression,
    variables: &mut HashMap<String, usize>,
    commands: &mut Vec<Command>,
    outer: Option<Operation>,
) -> Result<(), TranslationError> {
    let command_ = outer.clone().map(|op| Result::from(op.clone()));
    let command = match command_ {
        Some(result) => Some(result?),
        None => None,
    };
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
            ir_value(value, variables, None, commands)?;
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
        "vec" => ir_vector_operation(expression, variables, outer.clone(), commands)?,
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
            Some(Command::Get) => commands.push(Command::Dup),
            _ => (),
        }
        ir_value(value, variables, outer.clone(), commands)?;
        match command {
            Some(Command::HInsert) => (),
            Some(_) => {
                commands.push(command.clone().unwrap());
            }
            _ => panic!(
                "Unsupported function call found! Command {:?} not supported;",
                command
            ),
        }
        // special post-preparation for commands that put values on top of the stack (get command),
        match command {
            Some(Command::Get) => commands.push(Command::Swap),
            _ => (),
        }
    }
    match command {
        Some(Command::Get) => commands.push(Command::Del),
        Some(Command::HInsert) => commands.push(command.clone().unwrap()),
        _ => (),
    }
    Ok(())
}
