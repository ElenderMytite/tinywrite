use crate::ir::Command;
use std::{collections::HashMap, fmt::Debug};
mod instructions;
mod reference_counting;
mod types;
pub use types::*;
impl From<TypeError> for ExecutionError {
    fn from(value: TypeError) -> Self {
        Self::TypeMismatch(value)
    }
}
#[derive(Debug, Clone, Copy)]
pub enum ExecutionError {
    TypeMismatch(TypeError),
    ConversionError { from: Type, to: Type },
    StackUnderflow,
}
pub struct VM {
    /// instruction pointer
    pub ip: usize,
    flush: bool,
    pub code: Vec<Command>,
    /// storage for temporary calculations
    pub stack: Vec<StackValue>,
    /// storage for variables
    pub vars: Vec<StackValue>,
    /// how many references to heap are in stack and vars,
    pub(self) vars_refs: HashMap<usize, usize>,
    pub(self) stack_refs: HashMap<usize, usize>,
    /// storage for large data (vector, hash map or a string)
    next_addr: usize,
    /// When the value moves from stack to heap, do not decrease amount of pointers to a value,
    pub heap: HashMap<usize, HeapItem>,
}
impl VM {
    pub fn stack_pop(&mut self) -> Result<StackValue, ExecutionError> {
        let value = self.stack.pop();
        match value {
            Some(x) => Ok(x),
            None => Err(ExecutionError::StackUnderflow),
        }
    }
    pub fn stack_top(&mut self) -> Result<StackValue, ExecutionError> {
        let value = self.stack.last();
        match value {
            Some(x) => Ok(*x),
            None => Err(ExecutionError::StackUnderflow),
        }
    }
    pub fn new(code: Vec<Command>) -> Self {
        Self {
            ip: 0,
            flush: false,
            code: code,
            vars: Vec::new(),
            vars_refs: HashMap::new(),
            stack: Vec::new(),
            stack_refs: HashMap::new(),
            next_addr: 0,
            heap: HashMap::new(),
        }
    }
    /// prints debugging information before executing each command if called with debug = true
    pub fn execute_program(
        &mut self,
        debug: bool,
        clear_stack: bool,
    ) -> Result<usize, ExecutionError> {
        let mut exit = 0;
        while self.ip < self.code.len() && exit == 0 {
            if debug {
                eprintln!(
                    "{}: {:?};\n stack: {:?};\n refs{:?}",
                    self.ip,
                    self.code[self.ip].clone(),
                    self.stack.clone(),
                    self.debug_refs(debug)
                )
            }
            self.execute_command(&mut exit)?;
        }
        if self.flush {
            println!()
        }
        if clear_stack {
            self.clear_stack(debug);
        }
        Ok(exit - 1)
        // garbage collection
    }
    /// reads command at instruction pointer (ip), and calls corresponding function, then increases instruction pointer by one
    fn execute_command(&mut self, exit: &mut usize) -> Result<(), ExecutionError> {
        let command = self.code[self.ip];
        self.ip += 1;
        match command {
            Command::VNew => Ok(self.new_vec()),
            Command::Group => self.group(),
            Command::Prepend => self.prepend(),
            Command::HNew => Ok(self.new_hmap()),
            Command::Add => self.add(),
            Command::Sub => self.sub(),
            Command::Mul => self.mul(),
            Command::Div => self.div(),
            Command::Mod => self.modd(),
            Command::Byte => self.byte(),
            Command::Char => self.char(),
            Command::Dup => self.dup(),
            Command::Swap => self.swap(),
            Command::Del => self.del(),
            Command::Put(value) => Ok(self.put(value)),
            Command::Call(code) => match code {
                1 => self.dump(),
                x => Ok(*exit = x + 1),
            },
            Command::Eq => self.eq(),
            Command::Neq => self.neq(),
            Command::Geq => self.geq(),
            Command::Leq => self.leq(),
            Command::Gt => self.gt(),
            Command::Ls => self.ls(),
            Command::Not => self.not(),
            Command::And => self.and(),
            Command::Or => self.or(),
            Command::Xor => self.xor(),
            Command::Nor => self.nor(),
            Command::Nand => self.nand(),
            Command::Load(addr) => Ok(self.load(addr)),
            Command::Store(addr) => self.store(addr),
            Command::Jmp(addr) => self.jump(addr),
            Command::Get => self.get(),
            Command::HContains => self.hmap_contains(),
            Command::Len => self.len(),
            Command::VPop => self.vec_pop(),
            Command::HRemove => self.hmap_remove(),
            Command::VPush => self.vec_push(),
            Command::HInsert => self.hmap_insert(),
        }
    }
}
