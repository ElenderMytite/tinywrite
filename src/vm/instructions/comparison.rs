use crate::vm::{ExecutionError, StackValue, VM};
impl VM {
    pub fn eq(&mut self) -> Result<(), ExecutionError> {
        let b = self.stack_pop()?.int()?;
        let a = self.stack_pop()?.int()?;
        self.stack.push(StackValue::Bool(a == b));
        Ok(())
    }
    pub fn neq(&mut self) -> Result<(), ExecutionError> {
        let b = self.stack_pop()?.int()?;
        let a = self.stack_pop()?.int()?;
        self.stack.push(StackValue::Bool(a != b));
        Ok(())
    }
    pub fn geq(&mut self) -> Result<(), ExecutionError> {
        let b = self.stack_pop()?.int()?;
        let a = self.stack_pop()?.int()?;
        self.stack.push(StackValue::Bool(a >= b));
        Ok(())
    }
    pub fn leq(&mut self) -> Result<(), ExecutionError> {
        let b = self.stack_pop()?.int()?;
        let a = self.stack_pop()?.int()?;
        self.stack.push(StackValue::Bool(a <= b));
        Ok(())
    }
    pub fn ls(&mut self) -> Result<(), ExecutionError> {
        let b = self.stack_pop()?.int()?;
        let a = self.stack_pop()?.int()?;
        self.stack.push(StackValue::Bool(a < b));
        Ok(())
    }
    pub fn gt(&mut self) -> Result<(), ExecutionError> {
        let b = self.stack_pop()?.int()?;
        let a = self.stack_pop()?.int()?;
        self.stack.push(StackValue::Bool(a > b));
        Ok(())
    }
}
