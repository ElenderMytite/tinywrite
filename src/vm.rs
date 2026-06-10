use crate::ir::Command;
use std::cmp::max;
mod instructions;
pub(crate) struct VM {
    pub ip: usize,
    pub flush: bool,
    pub code: Vec<Command>,
    pub stack: Vec<StackValue>,
    pub heap: Vec<HeapValue>,
    pub env: Vec<StackValue>,
}
impl VM {
    pub fn new(code: Vec<Command>) -> Self {
        Self {
            ip: 0,
            flush: false,
            code: code,
            stack: Vec::new(),
            heap: Vec::new(),
            env: Vec::new(),
        }
    }
    pub fn execute(&mut self) {
        while self.ip < self.code.len() {
            match self.code[self.ip].clone() {
                Command::Add => self.add(),
                Command::Sub => self.sub(),
                Command::Mul => self.mul(),
                Command::Div => self.div(),
                Command::Mod => self.modd(),
                Command::Byte => self.byte(),
                Command::Char => self.char(),
                Command::Put(stack_value) => {
                    self.stack.push(stack_value);
                }
                Command::Dup => {
                    assert!(!self.stack.is_empty());
                    self.stack.push(self.stack.last().unwrap().clone());
                }
                Command::Del => {
                    assert!(!self.stack.is_empty());
                    self.stack.pop();
                }
                Command::Swap => {
                    assert!(self.stack.len() >= 2);
                    let b = self.stack.pop().unwrap();
                    let a = self.stack.pop().unwrap();
                    self.stack.push(b);
                    self.stack.push(a);
                }
                Command::Cls => {
                    if self.flush {
                        println!();
                        self.flush = false;
                    }
                    self.stack.clear();
                }
                Command::Print => {
                    assert!(self.stack.len() >= 1);
                    self.flush = true;
                    let value = self.stack.pop().unwrap();
                    if value == StackValue::Char('\n') {
                        println!();
                        self.flush = false;
                    } else {
                        print!("{} ", print_value(&value));
                    }
                }
                Command::Eq => self.eq(),
                Command::Neq => self.neq(),
                Command::Geq => self.geq(),
                Command::Leq => self.leq(),
                Command::Gt => self.gt(),
                Command::Ls => self.ls(),
                Command::Not => {
                    assert!(self.stack.len() >= 1);
                    let a = self.stack.pop().unwrap().bool().unwrap();
                    self.stack.push(StackValue::Bool(!a));
                }
                Command::And => {
                    assert!(self.stack.len() >= 2);
                    let (b, a) = (
                        self.stack.pop().unwrap().bool().unwrap(),
                        self.stack.pop().unwrap().bool().unwrap(),
                    );
                    self.stack.push(StackValue::Bool(a && b));
                }
                Command::Or => {
                    assert!(self.stack.len() >= 2);
                    let (b, a) = (
                        self.stack.pop().unwrap().bool().unwrap(),
                        self.stack.pop().unwrap().bool().unwrap(),
                    );
                    self.stack.push(StackValue::Bool(a || b));
                }
                Command::Xor => {
                    assert!(self.stack.len() >= 2);
                    let (b, a) = (
                        self.stack.pop().unwrap().bool().unwrap(),
                        self.stack.pop().unwrap().bool().unwrap(),
                    );
                    self.stack.push(StackValue::Bool(a != b));
                }
                Command::Nor => {
                    assert!(self.stack.len() >= 2);
                    let (b, a) = (
                        self.stack.pop().unwrap().bool().unwrap(),
                        self.stack.pop().unwrap().bool().unwrap(),
                    );
                    self.stack.push(StackValue::Bool(!(a || b)));
                }
                Command::Nand => {
                    assert!(self.stack.len() >= 2);
                    let (b, a) = (
                        self.stack.pop().unwrap().bool().unwrap(),
                        self.stack.pop().unwrap().bool().unwrap(),
                    );
                    self.stack.push(StackValue::Bool(!(a && b)));
                }
                Command::Load(adress) => {
                    if adress >= self.env.len() {
                        self.stack.push(StackValue::Nil);
                    } else {
                        self.stack.push(self.env[adress].clone());
                    }
                }
                Command::Store(adress) => {
                    assert!(self.stack.len() >= 1);
                    self.env
                        .resize(max(adress + 1, self.env.len()), StackValue::Nil);
                    self.env[adress] = self.stack.pop().unwrap();
                }
                Command::Jmp(adress) => {
                    assert!(self.stack.len() >= 1);
                    if self.stack.pop().unwrap().bool().unwrap() {
                        self.ip = adress;
                        continue;
                    }
                }
                Command::Len => {
                    assert!(self.stack.len() >= 1);
                    let vector = &self.heap[self.stack.pop().unwrap().ptr().unwrap()];
                    match vector {
                        HeapValue::Vector(vec) => {
                            self.stack.push(StackValue::Int(vec.len() as isize));
                        } // HeapValue::HMap(map) => {
                          //     self.stack.push(StackValue::Int(map.len() as isize));
                          // }
                    }
                }
                Command::VNew => {
                    let ptr = self.heap.len();
                    self.heap.push(HeapValue::Vector(vec![]));
                    self.stack.push(StackValue::Ptr(ptr));
                }
                // Command::HNew => {
                //     let ptr = self.heap.len();
                //     self.heap.push(HeapValue::HMap(HashMap::new()));
                //     self.stack.push(StackValue::Ptr(ptr));
                // }
                Command::Push => {
                    assert!(self.stack.len() >= 2);
                    let element = self.stack.pop().unwrap();
                    let vector = &mut self.heap[self.stack.last().unwrap().ptr().unwrap()];
                    match vector {
                        HeapValue::Vector(vec) => {
                            vec.push(element);
                        }
                    }
                }
                Command::VPop => {
                    assert!(self.stack.len() >= 1);
                    let vector = &mut self.heap[self.stack.pop().unwrap().ptr().unwrap()];
                    match vector {
                        HeapValue::Vector(vec) => {
                            let element = vec.pop().unwrap();
                            self.stack.push(element);
                        }
                    }
                }
                Command::Get => {
                    assert!(self.stack.len() >= 2);
                    let index = self.stack.pop().unwrap();
                    let vector = &self.heap[self.stack.pop().unwrap().ptr().unwrap()];
                    match vector {
                        HeapValue::Vector(vec) => {
                            let index = index.int().unwrap();
                            // handle negative indices by wrapping around from the end of the vector
                            let idx = ((index % (vec.len() as isize) + vec.len() as isize)
                                % vec.len() as isize)
                                as usize;
                            let element = vec[idx];
                            self.stack.push(element);
                        } // HeapValue::HMap(map) => {
                          //     let element = map.get(&index).unwrap_or(&StackValue::Nil);
                          //     self.stack.push(element.clone());
                          // }
                    }
                }
            }
            self.ip += 1;
        }
    }
}
pub struct TypeError;
impl std::fmt::Debug for TypeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Value popped from the self.stack has an unexpected type!"
        )
    }
}
#[derive(Debug, Clone)]
pub enum HeapValue {
    Vector(Vec<StackValue>),
    // HMap(HashMap<StackValue, StackValue>),
    // Str(String),
}
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum StackValue {
    Bool(bool),
    Int(isize),
    Char(char),
    Ptr(usize), // index in the self.heap
    Nil,
}
impl StackValue {
    pub fn int(&self) -> Result<isize, TypeError> {
        match &self {
            Self::Int(x) => Ok(*x),
            _ => Err(TypeError),
        }
    }
    pub fn bool(&self) -> Result<bool, TypeError> {
        match &self {
            Self::Bool(b) => Ok(*b),
            _ => Err(TypeError),
        }
    }
    pub fn char(&self) -> Result<char, TypeError> {
        match &self {
            Self::Char(c) => Ok(*c),
            _ => Err(TypeError),
        }
    }
    pub fn ptr(&self) -> Result<usize, TypeError> {
        match &self {
            Self::Ptr(p) => Ok(*p),
            _ => Err(TypeError),
        }
    }
}
pub(super) fn print_value(value: &StackValue) -> String {
    match value {
        StackValue::Nil => "Nil".to_string(),
        StackValue::Int(x) => x.to_string(),
        StackValue::Bool(b) => b.to_string(),
        StackValue::Char(c) => c.to_string(),
        StackValue::Ptr(p) => format!("Ptr({p})"),
    }
}
