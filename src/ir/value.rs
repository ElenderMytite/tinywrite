use super::expression::ir_expression;
use super::*;
use crate::ir::Command;
use crate::parser::types::Value;
use crate::vm::StackValue;
use std::collections::HashMap;
pub(super) fn ir_value(
    value: &Value,
    variables: &mut HashMap<String, usize>,
    index: usize,
    outer: Option<Operation>,
) -> Result<Vec<Command>, ParseError> {
    let mut commands = Vec::new();
    match value {
        Value::Name(s) => {
            register_variable(variables, s.clone());
            commands.push(Command::Load(variables[s]));
        }
        Value::Bool(b) => commands.push(Command::Put(StackValue::Bool(*b))),
        Value::Number(x) => commands.push(Command::Put(StackValue::Int(*x))),
        Value::Char(c) => commands.push(Command::Put(StackValue::Char(*c))),
        Value::Expression(expr) => {
            commands.append(&mut ir_expression(
                expr,
                variables,
                index + commands.len(),
                outer,
            )?);
        }
    }
    Ok(commands)
}
