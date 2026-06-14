use std::io::{Write, stdout};

use crate::vm::{ExecutionError, StackValue, VM, format_value};
impl VM {
    pub fn dup(&mut self) -> Result<(), ExecutionError> {
        let a = self.stack_top()?;
        if let Ok(ptr) = a.ptr() {
            self.ref_from_stack(ptr);
        }
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
        let _ = self.stack_pop()?.ptr().inspect(|ptr| {
            self.drop_from_stack(*ptr, 1);
        });
        Ok(())
    }
    pub fn clear_stack(&mut self, debug: bool) {
        if self.flush {
            stdout().flush().unwrap();
            self.flush = false;
        }
        if debug {
            eprintln!("clearing stack");
        }
        for (reference, count) in self.stack_refs.clone().iter() {
            if debug {
                eprintln!(
                    "reducing {count} out of {} referecing of {reference}",
                    self.heap.get(&reference).unwrap().refs
                );
            }
            self.drop_from_stack(*reference, *count);
            self.stack_refs.remove(reference);
        }
        self.stack.clear();
    }
    pub fn debug_refs(&self, debug: bool) {
        if debug {
            eprintln!(
                "stack refs: {:?}; vars refs: {:?}, heap:  {:#?}",
                self.stack_refs.clone(),
                self.vars_refs.clone(),
                self.heap.clone()
            );
        } else {
            eprintln!("debug funciton called in normal mode");
        }
    }
    pub fn dump(&mut self) -> Result<(), ExecutionError> {
        let value = self.stack_pop()?;
        if let StackValue::Char('\n') = value {
            println!();
            self.flush = false;
        } else {
            print!("{}", format_value(&value, &self));
            self.flush = true;
        }
        Ok(())
    }
    pub fn put(&mut self, value: StackValue) {
        if let StackValue::Pointer(ptr) = value {
            self.ref_from_stack(ptr);
        }
        self.stack.push(value);
    }
    pub fn load(&mut self, addr: usize) {
        let value = *self.vars.get(addr).unwrap_or(&StackValue::Nil);
        if let Ok(ptr) = value.ptr() {
            self.ref_from_stack(ptr); // copied, not moved
        }
        println!("pushing {value:?}");
        self.stack.push(value);
    }
    pub fn store(&mut self, addr: usize) -> Result<(), ExecutionError> {
        let value = self.stack_pop()?;
        if addr >= self.vars.len() {
            self.vars.resize(addr + 1, StackValue::Nil);
        }
        let old_value = self.vars.get(addr);
        if let Some(StackValue::Pointer(address)) = old_value {
            self.drop_from_vars(*address);
        }
        self.vars[addr] = value;
        if let StackValue::Pointer(pointer) = value {
            self.ref_from_vars(pointer);
            self.drop_from_stack(pointer, 1);
        }
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
