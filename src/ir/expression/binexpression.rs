use std::collections::HashMap;

use crate::{
    TranslationError,
    ir::{Command, Commands, value::ir_value},
    parser::types::BinaryExpression,
};

pub(super) fn ir_binexpr(
    expression: &BinaryExpression,
    variables: &mut HashMap<String, usize>,
    commands: &mut Commands,
    strings: &mut Vec<String>,
) -> Result<(), TranslationError> {
    let o = expression.operation.clone();
    ir_value(&expression.left, variables, o.clone(), commands, strings)?;
    ir_value(&expression.right, variables, o.clone(), commands, strings)?;
    match o.clone() {
        Some(op) => commands.push(Command::try_from(op)?),
        None => (),
    }
    Ok(())
}
