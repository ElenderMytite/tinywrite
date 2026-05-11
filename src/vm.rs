use crate::ir::Command;
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StackValue {
    Bool(bool),
    Int(isize),
    Vector(Rc<RefCell<Vec<StackValue>>>),
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
        // println!("stack: {stack:?}");
        // println!("ip: {}", ip);
        // println!("op: {:?}", code[ip].clone());
        match code[ip].clone() {
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
            Command::Dup => {
                assert!(!stack.is_empty());
                stack.push(stack.last().unwrap().clone());
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
                stack.push(StackValue::Bool(a <= b));
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
                stack.push(env[adress].clone());
            }
            Command::Store(adress) => {
                assert!(stack.len() >= 1);
                while adress >= env.len() {
                    env.resize(adress + 1, StackValue::None);
                }
                env[adress] = stack.pop().unwrap();
            }
            Command::Jmp(adress) => {
                assert!(stack.len() >= 1);
                if stack.pop().unwrap().bool().unwrap() {
                    ip = adress;
                    continue;
                }
            }
            Command::Len => {
                assert!(stack.len() >= 1);
                let vector = stack.pop().unwrap();
                match vector {
                    StackValue::Vector(vec) => {
                        stack.push(StackValue::Int(vec.borrow().len() as isize));
                    }
                    _ => panic!("expected vector on the stack"),
                }
            }
            Command::New => {
                stack.push(StackValue::Vector(Rc::new(RefCell::new(Vec::new()))));
            }
            Command::Push => {
                assert!(stack.len() >= 2);
                let element = stack.pop().unwrap();
                let vector = stack.last().unwrap();
                match vector {
                    StackValue::Vector(vec) => {
                        vec.borrow_mut().push(element);
                    }
                    _ => panic!("expected vector on the stack"),
                }
            }
            Command::Pop => {
                assert!(stack.len() >= 1);
                let vector = stack.last().unwrap();
                match vector {
                    StackValue::Vector(vec) => {
                        let element = vec.borrow_mut().pop().unwrap();
                        stack.push(element);
                    }
                    _ => panic!("expected vector on the stack"),
                }
            }
            Command::Get => {
                assert!(stack.len() >= 2);
                let index = stack.pop().unwrap().int().unwrap();
                let vector = stack.pop().unwrap();
                match vector {
                    StackValue::Vector(vec) => {
                        let element =
                            vec.borrow()[(index % (vec.borrow().len() as isize)) as usize].clone();
                        stack.push(element);
                    }
                    _ => panic!("expected vector on the stack"),
                }
            }
        }
        ip += 1;
    }
}
