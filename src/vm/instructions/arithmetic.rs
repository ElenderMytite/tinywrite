use super::super::{ExecutionError, StackValue, VM};
impl VM {
    pub fn add(&mut self) -> Result<(), ExecutionError> {
        let b = self.stack_pop()?.int()?;
        let a = self.stack_pop()?.int()?;
        self.stack.push(StackValue::Int(a + b));
        Ok(())
    }
    pub fn sub(&mut self) -> Result<(), ExecutionError> {
        let b = self.stack_pop()?.int()?;
        let a = self.stack_pop()?.int()?;
        self.stack.push(StackValue::Int(a - b));
        Ok(())
    }
    pub fn mul(&mut self) -> Result<(), ExecutionError> {
        let b = self.stack_pop()?.int()?;
        let a = self.stack_pop()?.int()?;
        self.stack.push(StackValue::Int(a * b));
        Ok(())
    }
    pub fn div(&mut self) -> Result<(), ExecutionError> {
        let b = self.stack_pop()?.int()?;
        let a = self.stack_pop()?.int()?;
        self.stack.push(StackValue::Int(a / b));
        Ok(())
    }
    pub fn modd(&mut self) -> Result<(), ExecutionError> {
        let b = self.stack_pop()?.int()?;
        let a = self.stack_pop()?.int()?;
        self.stack.push(StackValue::Int(a % b));
        Ok(())
    }
}
