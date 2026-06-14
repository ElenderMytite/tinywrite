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
    /// prints debugging information before executing each command
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
        Ok(exit)
        // garbage collection
    }
    /// reads command at instruction pointer ip, and calls corresponding function, then increases instruction pointer by one
    fn execute_command(&mut self, exit: &mut usize) -> Result<(), ExecutionError> {
        let command = self.code[self.ip];
        self.ip += 1;
        match command {
            Command::VNew => self.new_vec(),
            Command::Group => return self.group(),
            Command::Prepend => return self.prepend(),
            Command::HNew => self.new_hmap(),
            Command::Add => return self.add(),
            Command::Sub => return self.sub(),
            Command::Mul => return self.mul(),
            Command::Div => return self.div(),
            Command::Mod => return self.modd(),
            Command::Byte => return self.byte(),
            Command::Char => return self.char(),
            Command::Dup => return self.dup(),
            Command::Swap => return self.swap(),
            Command::Del => return self.del(),
            Command::Put(value) => self.put(value),
            Command::Call(code) => match code {
                1 => return self.dump(),
                x => *exit = x + 1,
            },
            Command::Eq => return self.eq(),
            Command::Neq => return self.neq(),
            Command::Geq => return self.geq(),
            Command::Leq => return self.leq(),
            Command::Gt => return self.gt(),
            Command::Ls => return self.ls(),
            Command::Not => return self.not(),
            Command::And => return self.and(),
            Command::Or => return self.or(),
            Command::Xor => return self.xor(),
            Command::Nor => return self.nor(),
            Command::Nand => return self.nand(),
            Command::Load(addr) => self.load(addr),
            Command::Store(addr) => return self.store(addr),
            Command::Jmp(addr) => return self.jump(addr),
            Command::Get => return self.get(),
            Command::HContains => return self.hmap_contains(),
            Command::Len => return self.len(),
            Command::VPop => return self.vec_pop(),
            Command::HRemove => return self.hmap_remove(),
            Command::VPush => return self.vec_push(),
            Command::HInsert => return self.hmap_insert(),
        }
        Ok(())
    }
}
