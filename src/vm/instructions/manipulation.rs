use std::io::{Write, stdout};

use crate::vm::{ExecutionError, PrimitiveValue, StackValue, VM};
impl VM {
    pub fn dup(&mut self) -> Result<(), ExecutionError> {
        let a = self.stack_top()?.clone();
        self.stack.push(a);
        Ok(())
    }
    pub fn swap(&mut self) -> Result<(), ExecutionError> {
        let a = self.stack_pop()?;
        let b = self.stack_pop()?;
        self.stack.push(a);
        self.stack.push(b);
        Ok(())
    }
    pub fn del(&mut self) -> Result<(), ExecutionError> {
        self.stack_pop().map(|_| ())
    }
    pub fn clear_stack(&mut self, debug: bool) {
        if self.flush {
            stdout().flush().unwrap();
            self.flush = false;
        }
        if debug {
            eprintln!("clearing stack");
        }
        self.stack.clear();
    }
    pub fn put(&mut self, value: PrimitiveValue) {
        self.stack.push(StackValue::Primitive(value));
    }
    pub fn load(&mut self, addr: usize) {
        let void = StackValue::default();
        let value = self.vars.get(addr).unwrap_or(&void);
        self.stack.push(value.clone());
    }
    pub fn store(&mut self, addr: usize) -> Result<(), ExecutionError> {
        let value = self.stack_pop()?;
        if self.vars.len() <= addr {
            self.vars.resize(addr + 1, StackValue::default());
        }
        self.vars[addr] = value;
        Ok(())
    }
    pub fn jump(&mut self, addr: usize) -> Result<(), ExecutionError> {
        let condition = self.stack_pop()?;
        if !condition.bool().unwrap() {
            return Ok(());
        }
        if addr >= self.code.len() {
            panic!("Jump address out of bounds: {}", addr);
        }
        self.ip = addr;
        Ok(())
    }
}
