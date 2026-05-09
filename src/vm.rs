use crate::ir::Command;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StackValue {
    Bool(bool),
    Int(isize),
    None,
}
impl StackValue {
    fn int(&self) -> Result<isize, ()> {
        match &self {
            Self::Int(x) => Ok(*x),
            _ => Err(()),
        }
    }
    fn bool(&self) -> Result<bool, ()> {
        match &self {
            Self::Bool(b) => Ok(*b),
            _ => Err(()),
        }
    }
}
pub fn execute(code: &Vec<Command>, capacity: Option<usize>) {
    let mut ip = 0;
    let mut env: Vec<StackValue> = vec![StackValue::None; capacity.unwrap_or(0)];
    let mut stack: Vec<StackValue> = Vec::new();
    while ip < code.len() {
        match code[ip] {
            Command::Add => {
                assert!(stack.len() >= 2);
                let (b, a) = (
                    stack.pop().unwrap().int().unwrap(),
                    stack.pop().unwrap().int().unwrap(),
                );
                stack.push(StackValue::Int(a + b));
            }
            Command::Sub => {
                assert!(stack.len() >= 2);
                let (b, a) = (
                    stack.pop().unwrap().int().unwrap(),
                    stack.pop().unwrap().int().unwrap(),
                );
                stack.push(StackValue::Int(a - b));
            }
            Command::Mul => {
                assert!(stack.len() >= 2);
                let (b, a) = (
                    stack.pop().unwrap().int().unwrap(),
                    stack.pop().unwrap().int().unwrap(),
                );
                stack.push(StackValue::Int(a * b));
            }
            Command::Div => {
                assert!(stack.len() >= 2);
                let (b, a) = (
                    stack.pop().unwrap().int().unwrap(),
                    stack.pop().unwrap().int().unwrap(),
                );
                stack.push(StackValue::Int(a / b));
            }
            Command::Mod => {
                assert!(stack.len() >= 2);
                let (b, a) = (
                    stack.pop().unwrap().int().unwrap(),
                    stack.pop().unwrap().int().unwrap(),
                );
                stack.push(StackValue::Int(a % b));
            }
            Command::Put(stack_value) => {
                stack.push(stack_value);
            }
            Command::Cls => {
                for element in stack.iter() {
                    println!("{:?}", element);
                }
                stack.clear()
            }
            Command::Gt => {
                assert!(stack.len() >= 2);
                let (b, a) = (
                    stack.pop().unwrap().int().unwrap(),
                    stack.pop().unwrap().int().unwrap(),
                );
                stack.push(StackValue::Bool(a > b));
            }
            Command::Geq => {
                assert!(stack.len() >= 2);
                let (b, a) = (
                    stack.pop().unwrap().int().unwrap(),
                    stack.pop().unwrap().int().unwrap(),
                );
                stack.push(StackValue::Bool(a >= b));
            }
            Command::Ls => {
                assert!(stack.len() >= 2);
                let (b, a) = (
                    stack.pop().unwrap().int().unwrap(),
                    stack.pop().unwrap().int().unwrap(),
                );
                stack.push(StackValue::Bool(a < b));
            }
            Command::Leq => {
                assert!(stack.len() >= 2);
                let (b, a) = (
                    stack.pop().unwrap().int().unwrap(),
                    stack.pop().unwrap().int().unwrap(),
                );
                stack.push(StackValue::Bool(a >= b));
            }
            Command::Eq => {
                assert!(stack.len() >= 2);
                let (b, a) = (stack.pop().unwrap(), stack.pop().unwrap()); // works for both int and bool
                stack.push(StackValue::Bool(a == b));
            }
            Command::Neq => {
                assert!(stack.len() >= 2);
                let (b, a) = (stack.pop().unwrap(), stack.pop().unwrap()); // works for both int and bool
                stack.push(StackValue::Bool(a != b));
            }
            Command::Not => {
                assert!(stack.len() >= 2);
                let (b, a) = (
                    stack.pop().unwrap().bool().unwrap(),
                    stack.pop().unwrap().bool().unwrap(),
                );
                stack.push(StackValue::Bool(a != b));
            }
            Command::And => {
                assert!(stack.len() >= 2);
                let (b, a) = (
                    stack.pop().unwrap().bool().unwrap(),
                    stack.pop().unwrap().bool().unwrap(),
                );
                stack.push(StackValue::Bool(a != b));
            }
            Command::Or => {
                assert!(stack.len() >= 2);
                let (b, a) = (
                    stack.pop().unwrap().bool().unwrap(),
                    stack.pop().unwrap().bool().unwrap(),
                );
                stack.push(StackValue::Bool(a != b));
            }
            Command::Xor => {
                assert!(stack.len() >= 2);
                let (b, a) = (
                    stack.pop().unwrap().bool().unwrap(),
                    stack.pop().unwrap().bool().unwrap(),
                );
                stack.push(StackValue::Bool(a != b));
            }
            Command::Nor => {
                assert!(stack.len() >= 2);
                let (b, a) = (
                    stack.pop().unwrap().bool().unwrap(),
                    stack.pop().unwrap().bool().unwrap(),
                );
                stack.push(StackValue::Bool(a != b));
            }
            Command::Nand => {
                assert!(stack.len() >= 2);
                let (b, a) = (
                    stack.pop().unwrap().bool().unwrap(),
                    stack.pop().unwrap().bool().unwrap(),
                );
                stack.push(StackValue::Bool(a != b));
            }
            Command::Load(adress) => {
                stack.push(env[adress]);
            }
            Command::Store(adress) => {
                assert!(stack.len() >= 1);
                env[adress] = stack.pop().unwrap();
            }
        }
        ip += 1;
    }
}
