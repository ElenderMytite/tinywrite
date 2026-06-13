use crate::vm::{ExecutionError, StackValue, VM};
impl VM {
    pub fn and(&mut self) -> Result<(), ExecutionError> {
        let b = self.stack_pop()?.bool()?;
        let a = self.stack_pop()?.bool()?;
        self.stack.push(StackValue::Bool(a && b));
        Ok(())
    }
    pub fn or(&mut self) -> Result<(), ExecutionError> {
        let b = self.stack_pop()?.bool()?;
        let a = self.stack_pop()?.bool()?;
        self.stack.push(StackValue::Bool(a || b));
        Ok(())
    }
    pub fn not(&mut self) -> Result<(), ExecutionError> {
        let a = self.stack_pop()?.bool()?;
        self.stack.push(StackValue::Bool(!a));
        Ok(())
    }
    pub fn xor(&mut self) -> Result<(), ExecutionError> {
        let b = self.stack_pop()?.bool()?;
        let a = self.stack_pop()?.bool()?;
        self.stack.push(StackValue::Bool(a ^ b));
        Ok(())
    }
    pub fn nor(&mut self) -> Result<(), ExecutionError> {
        let b = self.stack_pop()?.bool()?;
        let a = self.stack_pop()?.bool()?;
        self.stack.push(StackValue::Bool(!(a || b)));
        Ok(())
    }
    pub fn nand(&mut self) -> Result<(), ExecutionError> {
        let b = self.stack_pop()?.bool()?;
        let a = self.stack_pop()?.bool()?;
        self.stack.push(StackValue::Bool(!(a && b)));
        Ok(())
    }
}
