use super::expression::ir_expression;
use super::value::ir_value;
use super::*;
use crate::parser::types::{Expression, Operation, Value, VectorOp};
use std::collections::HashMap;

pub(super) fn ir_vector_operation(
    iteration: &Expression,
    variables: &mut HashMap<String, usize>,
    outer: Option<Operation>,
    commands: &mut Commands,
    strings: &mut Vec<String>,
) -> Result<(), TranslationError> {
    match iteration.operation.clone() {
        Some(Operation::Vector(op)) => match op {
            VectorOp::Unpack => {
                // assume vector is on top of the stack
                // create variables for the index and the vector pointer
                let mut k_idx: usize = 0;
                let idx: usize;
                loop {
                    if variables.contains_key(&format!("--idx-{}", k_idx)) {
                        k_idx += 1;
                    } else {
                        idx = register_variable(variables, format!("--idx-{}", k_idx));
                        break;
                    }
                }
                let mut v_idx = 0;
                let vec_ptr: usize;
                loop {
                    if variables.contains_key(&format!("--vec-{}", k_idx)) {
                        v_idx += 1;
                    } else {
                        vec_ptr = register_variable(variables, format!("--vec-{}", k_idx));
                        break;
                    }
                }
                match outer {
                    Some(Operation::Computation(Computation::Add)) => {
                        commands.push(Command::Put(PrimitiveValue::Int(0)));
                    }
                    Some(Operation::Computation(Computation::Mul)) => {
                        commands.push(Command::Put(PrimitiveValue::Int(1)));
                    }
                    _ => (),
                }
                for value in iteration.left.iter().chain(iteration.right.iter()) {
                    let vector: usize = match value {
                        Value::Name(s) => register_variable(variables, s.clone()),
                        _ => panic!("Non-name node found inside vector unpacking!"),
                    };
                    commands.push(Command::Put(PrimitiveValue::Int(0)));
                    commands.push(Command::Store(idx));
                    commands.push(Command::Load(vector));
                    commands.push(Command::Store(vec_ptr));
                    // put neultral element to the stack before the first iteration (0 for addition, 1 for multiplication);
                    // Inverse operations first apply normal opertation on both sides and then apply the inverse operation to the current value on the stack, so we need to put the neutral element before the first iteration
                    match outer {
                        Some(Operation::Computation(Computation::Add | Computation::Sub)) => {
                            commands.push(Command::Put(PrimitiveValue::Int(0)));
                        }
                        Some(Operation::Computation(Computation::Mul | Computation::Div)) => {
                            commands.push(Command::Put(PrimitiveValue::Int(1)));
                        }
                        _ => (),
                    }
                    // for loop
                    let label = commands.len();
                    commands.push(Command::Load(vec_ptr));
                    commands.push(Command::Load(idx));
                    match outer {
                        Some(Operation::Comparison(_)) => {
                            todo!("vector unpacking inside comparison not supported")
                        }
                        Some(Operation::Computation(computation)) => match computation {
                            Computation::Add | Computation::Sub => {
                                // get value at idx and add it to the current value on the stack
                                commands.push(Command::Get);
                                commands.push(Command::Add);
                            }
                            Computation::Mul | Computation::Div => {
                                // get value at idx and multiply it to the current value on the stack
                                commands.push(Command::Get);
                                commands.push(Command::Mul);
                            }
                            _ => panic!("Unsupported computation operation for vector unpacking!"),
                        },
                        Some(Operation::Set) => {
                            panic!("Cannot use vector unpacking inside assignment!")
                        }
                        Some(Operation::Logic(logic_op)) => {
                            commands.push(Command::Get);
                            commands.push(Command::try_from(Operation::Logic(logic_op))?);
                        }
                        None => {
                            // just push the value at idx to the stack
                            commands.push(Command::Get);
                        }
                        _ => panic!("Unsupported outer operation for vector unpacking!"),
                    };
                    // exit condition: idx + 1 < len(vector) (next iteration will try to access idx + 1, so we need to check if it's out of bounds)
                    commands.push(Command::Load(idx));
                    commands.push(Command::Put(PrimitiveValue::Int(1)));
                    commands.push(Command::Add);
                    commands.push(Command::Dup);
                    commands.push(Command::Store(idx));
                    commands.push(Command::Load(vec_ptr));
                    commands.push(Command::Len);
                    commands.push(Command::Ls);
                    commands.push(Command::Jmp(label));
                }
                free_variable(variables, format!("--vec-{v_idx}"));
                free_variable(variables, format!("--idx-{k_idx}"));
            }
            VectorOp::Pack => {
                parse_pack(iteration, variables, commands, strings)?;
            }
        },
        Some(Operation::Call(call)) => match call.as_str() {
            "vec" => parse_pack(iteration, variables, commands, strings)?,
            _ => todo!(),
        },
        Some(_) => panic!("Non-vector operation found inside iteration!"),
        None => todo!(),
    }
    Ok(())
}
fn parse_pack(
    iteration: &Expression,
    variables: &mut HashMap<String, usize>,
    commands: &mut Commands,
    strings: &mut Vec<String>,
) -> Result<(), TranslationError> {
    commands.push(Command::VNew);
    for node in iteration.left.iter().chain(iteration.right.iter()) {
        match node {
            Value::Expression(expr) => {
                ir_expression(expr, variables, None, commands, strings)?;
            }
            _ => {
                ir_value(node, variables, None, commands, strings)?;
            }
        }
        commands.push(Command::VPush);
    }
    Ok(())
}
