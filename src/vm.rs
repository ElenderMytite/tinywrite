use crate::ir::Command;
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
    pub fn execute(&mut self) -> Result<(), TypeError> {
        while self.ip < self.code.len() {
            // eprintln!(
            //     "{}: {:?}; {:?}",
            //     self.ip,
            //     self.stack.clone(),
            //     self.code[self.ip].clone()
            // );
            let result = match self.code[self.ip].clone() {
                Command::Add => {
                    self.add();
                    Ok(())
                }
                Command::Sub => {
                    self.sub();
                    Ok(())
                }
                Command::Mul => {
                    self.mul();
                    Ok(())
                }
                Command::Div => {
                    self.div();
                    Ok(())
                }
                Command::Mod => {
                    self.modd();
                    Ok(())
                }
                Command::Byte => {
                    self.byte();
                    Ok(())
                }
                Command::Char => {
                    self.char();
                    Ok(())
                }
                Command::Cls => {
                    self.cls();
                    Ok(())
                }
                Command::Dup => {
                    self.dup();
                    Ok(())
                }
                Command::Swap => {
                    self.swap();
                    Ok(())
                }
                Command::Del => {
                    self.drop();
                    Ok(())
                }
                Command::Put(value) => {
                    self.put(value);
                    Ok(())
                }
                Command::Print => {
                    self.print();
                    Ok(())
                }
                Command::Eq => {
                    self.eq();
                    Ok(())
                }
                Command::Neq => {
                    self.neq();
                    Ok(())
                }
                Command::Geq => {
                    self.geq();
                    Ok(())
                }
                Command::Leq => {
                    self.leq();
                    Ok(())
                }
                Command::Gt => {
                    self.gt();
                    Ok(())
                }
                Command::Ls => {
                    self.ls();
                    Ok(())
                }
                Command::Not => {
                    self.not();
                    Ok(())
                }
                Command::And => {
                    self.and();
                    Ok(())
                }
                Command::Or => {
                    self.or();
                    Ok(())
                }
                Command::Xor => {
                    self.xor();
                    Ok(())
                }
                Command::Nor => {
                    self.nor();
                    Ok(())
                }
                Command::Nand => {
                    self.nand();
                    Ok(())
                }
                Command::Load(addr) => {
                    self.load(addr);
                    Ok(())
                }
                Command::Store(addr) => {
                    self.store(addr);
                    Ok(())
                }
                Command::Jmp(addr) => {
                    self.jmp(addr);
                    Ok(())
                }
                Command::Get => self.vec_get().map(|_| ()),
                Command::Len => self.len().map(|_| ()),
                Command::VNew => {
                    self.new_vec();
                    Ok(())
                }
                Command::VPop => self.vec_pop().map(|_| ()),
                Command::VPush => self.vec_push().map(|_| ()),
            };
            result?;
            self.ip += 1;
        }
        Ok(())
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
impl HeapValue {
    pub fn len(&self) -> usize {
        match self {
            HeapValue::Vector(vec) => vec.len(),
            // HeapValue::HMap(map) => map.len(),
            // HeapValue::Str(s) => s.len(),
        }
    }
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
pub(super) fn print_value(value: &StackValue, vm: &VM) -> String {
    match value {
        StackValue::Nil => "Nil".to_string(),
        StackValue::Int(x) => x.to_string(),
        StackValue::Bool(b) => b.to_string(),
        StackValue::Char(c) => c.to_string(),
        StackValue::Ptr(p) => {
            let heap_val = &vm.heap[*p];
            match heap_val {
                HeapValue::Vector(vec) => {
                    let elements: Vec<String> = vec.iter().map(|v| print_value(v, vm)).collect();
                    format!("[{}]", elements.join(", "))
                } // HeapValue::HMap(map) => {
                  //     let elements: Vec<String> = map
                  //         .iter()
                  //         .map(|(k, v)| format!("{}: {}", print_value(k, vm), print_value(v, vm)))
                  //         .collect();
                  //     format!("{{{}}}", elements.join(", "))
                  // }
                  // HeapValue::Str(s) => s.clone(),
            }
        }
    }
}
