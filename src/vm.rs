use crate::ir::Command;
use std::cell::RefCell;
use std::cmp::max;
use std::io::{self, Write};
use std::rc::Rc;
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StackValue {
    Bool(bool),
    Int(isize),
    Char(char),
    Vector(Rc<RefCell<Vec<StackValue>>>),
    Nil,
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
    fn char(&self) -> Result<char, ()> {
        match &self {
            Self::Char(c) => Ok(*c),
            _ => Err(()),
        }
    }
}
pub fn execute(code: &Vec<Command>, env: &mut Vec<StackValue>) {
    let mut ip = 0;
    let mut stack: Vec<StackValue> = Vec::new();
    let mut flush: bool = false;
    while ip < code.len() {
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
            Command::Byte => {
                assert!(stack.len() >= 1);
                let c = stack.pop().unwrap().char().unwrap();
                stack.push(StackValue::Int(c as isize));
            }
            Command::Char => {
                assert!(stack.len() >= 1);
                let i = stack.pop().unwrap().int().unwrap();
                stack.push(StackValue::Char(char::from_u32(i as u32).unwrap()));
            }
            Command::Put(stack_value) => {
                stack.push(stack_value);
            }
            Command::Dup => {
                assert!(!stack.is_empty());
                stack.push(stack.last().unwrap().clone());
            }
            Command::Del => {
                assert!(!stack.is_empty());
                stack.pop();
            }
            Command::Swap => {
                assert!(stack.len() >= 2);
                let b = stack.pop().unwrap();
                let a = stack.pop().unwrap();
                stack.push(b);
                stack.push(a);
            }
            Command::Cls => {
                if flush {
                    println!();
                    flush = false;
                }
                stack.clear();
            }
            Command::Print => {
                assert!(stack.len() >= 1);
                flush = true;
                let value = stack.pop().unwrap();
                if value == StackValue::Char('\n') {
                    println!();
                    flush = false;
                } else {
                    print!("{} ", print_value(&value));
                }
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
                assert!(stack.len() >= 1);
                let a = stack.pop().unwrap().bool().unwrap();
                stack.push(StackValue::Bool(!a));
            }
            Command::And => {
                assert!(stack.len() >= 2);
                let (b, a) = (
                    stack.pop().unwrap().bool().unwrap(),
                    stack.pop().unwrap().bool().unwrap(),
                );
                stack.push(StackValue::Bool(a && b));
            }
            Command::Or => {
                assert!(stack.len() >= 2);
                let (b, a) = (
                    stack.pop().unwrap().bool().unwrap(),
                    stack.pop().unwrap().bool().unwrap(),
                );
                stack.push(StackValue::Bool(a || b));
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
                stack.push(StackValue::Bool(!(a || b)));
            }
            Command::Nand => {
                assert!(stack.len() >= 2);
                let (b, a) = (
                    stack.pop().unwrap().bool().unwrap(),
                    stack.pop().unwrap().bool().unwrap(),
                );
                stack.push(StackValue::Bool(!(a && b)));
            }
            Command::Load(adress) => {
                if adress >= env.len() {
                    stack.push(StackValue::Nil);
                } else {
                    stack.push(env[adress].clone());
                }
            }
            Command::Store(adress) => {
                assert!(stack.len() >= 1);
                env.resize(max(adress + 1, env.len()), StackValue::Nil);
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
            Command::VNew => {
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
            Command::VPop => {
                assert!(stack.len() >= 1);
                let vector = stack.pop().unwrap();
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
                        let element = vec.borrow()[((index % (vec.borrow().len() as isize)
                            + vec.borrow().len() as isize)
                            % vec.borrow().len() as isize)
                            as usize]
                            .clone();
                        stack.push(element);
                    }
                    val => panic!("expected vector on the stack, found {val:?}"),
                }
            }
        }
        ip += 1;
    }
}
pub(super) fn print_value(value: &StackValue) -> String {
    match value {
        StackValue::Nil => "Nil".to_string(),
        StackValue::Int(x) => x.to_string(),
        StackValue::Bool(b) => b.to_string(),
        StackValue::Char(c) => c.to_string(),
        StackValue::Vector(vec) => format!(
            "[{}]",
            vec.borrow()
                .iter()
                .map(|v| print_value(v))
                .collect::<Vec<String>>()
                .join(", ")
        ),
    }
}
