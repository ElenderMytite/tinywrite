use std::collections::HashMap;
mod binexpression;
mod multiexpression;

use crate::{
    TranslationError,
    ir::Commands,
    parser::types::{ExpressionNode, Operation},
};

pub fn ir_expression(
    expr: &ExpressionNode,
    variables: &mut HashMap<String, usize>,
    outer: Option<Operation>,
    commands: &mut Commands,
    strings: &mut Vec<String>,
) -> Result<(), TranslationError> {
    match expr {
        ExpressionNode::Multi(folder) => {
            multiexpression::ir_multiexpr(&folder, variables, outer, commands, strings)
        }

        ExpressionNode::Binary(binexpr) => {
            binexpression::ir_binexpr(binexpr, variables, commands, strings)
        }
    }
}
