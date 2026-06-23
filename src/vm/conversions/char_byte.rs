use crate::vm::{ExecutionError, PrimitiveValue, Type, VM};
impl VM {
    pub fn byte(&mut self) -> Result<(), ExecutionError> {
        let c = self.stack.pop().unwrap().char().unwrap();
        self.put(PrimitiveValue::Int(c as isize));
        Ok(())
    }
    pub fn char(&mut self) -> Result<(), ExecutionError> {
        let i = self.stack.pop().unwrap().int()?;
        self.put(PrimitiveValue::Char(checked_char(i as u32)?));
        Ok(())
    }
}
fn checked_char(x: u32) -> Result<char, ExecutionError> {
    match char::from_u32(x) {
        Some(c) => Ok(c),
        None => Err(ExecutionError::ConversionError {
            from: Type::Int,
            to: Type::Char,
        }),
    }
}
