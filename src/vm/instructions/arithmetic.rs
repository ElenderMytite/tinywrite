use super::super::StackValue;
use super::super::VM;
impl VM {
    pub fn add(&mut self) {
        let b = self.stack.pop().unwrap().int().unwrap();
        let a = self.stack.pop().unwrap().int().unwrap();
        self.stack.push(StackValue::Int(a + b));
    }
    pub fn sub(&mut self) {
        let b = self.stack.pop().unwrap().int().unwrap();
        let a = self.stack.pop().unwrap().int().unwrap();
        self.stack.push(StackValue::Int(a - b));
    }
    pub fn mul(&mut self) {
        let b = self.stack.pop().unwrap().int().unwrap();
        let a = self.stack.pop().unwrap().int().unwrap();
        self.stack.push(StackValue::Int(a * b));
    }
    pub fn div(&mut self) {
        let b = self.stack.pop().unwrap().int().unwrap();
        let a = self.stack.pop().unwrap().int().unwrap();
        self.stack.push(StackValue::Int(a / b));
    }
    pub fn modd(&mut self) {
        let b = self.stack.pop().unwrap().int().unwrap();
        let a = self.stack.pop().unwrap().int().unwrap();
        self.stack.push(StackValue::Int(a % b));
    }
}
