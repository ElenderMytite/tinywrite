use super::{Command, Commands, register_variable};
use crate::ir::TranslationError;
use crate::ir::expression::ir_expression;
use crate::parser::types::{Operation, Value};
use crate::vm::PrimitiveValue;
use std::collections::HashMap;
pub(super) fn ir_value(
    value: &Value,
    variables: &mut HashMap<String, usize>,
    outer: Option<Operation>,
    commands: &mut Commands,
    strings: &mut Vec<String>,
) -> Result<(), TranslationError> {
    match value {
        Value::Name(s) => {
            register_variable(variables, s.clone());
            commands.push(Command::Load(variables[s]));
        }
        Value::Bool(b) => commands.push(Command::Put(PrimitiveValue::Bool(*b))),
        Value::Number(x) => commands.push(Command::Put(PrimitiveValue::Int(*x))),
        Value::Char(c) => commands.push(Command::Put(PrimitiveValue::Char(*c))),
        Value::Expression(expr) => {
            ir_expression(expr, variables, outer, commands, strings)?;
        }
        Value::Literal(literal) => {
            let ptr = strings.len();
            strings.push(literal.clone());
            commands.push(Command::PutS(ptr));
        }
    }
    Ok(())
}
