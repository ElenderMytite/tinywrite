use crate::vm::{StackValue, VM};
impl VM {
    pub fn eq(&mut self) {
        let b = self.stack.pop().unwrap().int().unwrap();
        let a = self.stack.pop().unwrap().int().unwrap();
        self.stack.push(StackValue::Bool(a == b));
    }
    pub fn neq(&mut self) {
        let b = self.stack.pop().unwrap().int().unwrap();
        let a = self.stack.pop().unwrap().int().unwrap();
        self.stack.push(StackValue::Bool(a != b));
    }
    pub fn geq(&mut self) {
        let b = self.stack.pop().unwrap().int().unwrap();
        let a = self.stack.pop().unwrap().int().unwrap();
        self.stack.push(StackValue::Bool(a >= b));
    }
    pub fn leq(&mut self) {
        let b = self.stack.pop().unwrap().int().unwrap();
        let a = self.stack.pop().unwrap().int().unwrap();
        self.stack.push(StackValue::Bool(a <= b));
    }
    pub fn ls(&mut self) {
        let b = self.stack.pop().unwrap().int().unwrap();
        let a = self.stack.pop().unwrap().int().unwrap();
        self.stack.push(StackValue::Bool(a < b));
    }
    pub fn gt(&mut self) {
        let b = self.stack.pop().unwrap().int().unwrap();
        let a = self.stack.pop().unwrap().int().unwrap();
        self.stack.push(StackValue::Bool(a > b));
    }
}
