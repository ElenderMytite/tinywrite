use crate::vm::{StackValue, VM, print_value};
impl VM {
    pub fn dup(&mut self) {
        let a = self.stack.last().unwrap().clone();
        self.stack.push(a);
    }
    pub fn swap(&mut self) {
        let a = self.stack.pop().unwrap();
        let b = self.stack.pop().unwrap();
        self.stack.push(a);
        self.stack.push(b);
    }
    pub fn drop(&mut self) {
        self.stack.pop();
    }
    pub fn cls(&mut self) {
        if self.flush {
            println!();
            self.flush = false;
        }
        self.stack.clear();
    }
    pub fn print(&mut self) {
        let value = self.stack.pop();
        if value.is_none() {
            return;
        }
        let value = value.unwrap();
        print!("{}", print_value(&value, &self));
        self.flush = true;
    }
    pub fn put(&mut self, value: StackValue) {
        self.stack.push(value);
    }
    pub fn load(&mut self, addr: usize) {
        let value = self.env.get(addr).unwrap_or(&StackValue::Nil).clone();
        self.stack.push(value);
    }
    pub fn store(&mut self, addr: usize) {
        let value = self.stack.pop().unwrap();
        if addr >= self.stack.len() {
            self.env.resize(addr + 1, StackValue::Nil);
        }
        self.env[addr] = value;
    }
    pub fn jmp(&mut self, addr: usize) {
        let condition = self.stack.pop().unwrap();
        if !condition.bool().unwrap() {
            eprintln!("breaking out of block at {}", self.ip);
            return;
        }
        if addr >= self.code.len() {
            panic!("Jump address out of bounds: {}", addr);
        }
        eprintln!("jump to {addr}");
        self.ip = addr;
        self.ip -= 1; // revert ip increase in main
    }
}
