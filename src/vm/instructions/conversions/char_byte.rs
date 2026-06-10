use crate::vm::StackValue;
use crate::vm::VM;
impl VM {
    pub fn byte(&mut self) {
        let c = self.stack.pop().unwrap().char().unwrap();
        self.stack.push(StackValue::Int(c as isize));
    }
    pub fn char(&mut self) {
        let i = self.stack.pop().unwrap().int().unwrap();
        self.stack.push(StackValue::Char(i as u8 as char));
    }
}
