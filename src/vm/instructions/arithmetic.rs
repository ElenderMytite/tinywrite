use super::super::{ExecutionError, PrimitiveValue, VM};
impl VM {
    pub fn add(&mut self) -> Result<(), ExecutionError> {
        let b = { self.stack_pop()?.int()? };
        let a = self.stack_pop()?.int()?;
        self.put(PrimitiveValue::Int(a + b));
        Ok(())
    }
    pub fn sub(&mut self) -> Result<(), ExecutionError> {
        let b = self.stack_pop()?.int()?;
        let a = self.stack_pop()?.int()?;
        self.put(PrimitiveValue::Int(a - b));
        Ok(())
    }
    pub fn mul(&mut self) -> Result<(), ExecutionError> {
        let b = self.stack_pop()?.int()?;
        let a = self.stack_pop()?.int()?;
        self.put(PrimitiveValue::Int(a * b));
        Ok(())
    }
    pub fn div(&mut self) -> Result<(), ExecutionError> {
        let b = self.stack_pop()?.int()?;
        let a = self.stack_pop()?.int()?;
        self.put(PrimitiveValue::Int(a / b));
        Ok(())
    }
    pub fn modd(&mut self) -> Result<(), ExecutionError> {
        let b = self.stack_pop()?.int()?;
        let a = self.stack_pop()?.int()?;
        self.put(PrimitiveValue::Int(a % b));
        Ok(())
    }
}
