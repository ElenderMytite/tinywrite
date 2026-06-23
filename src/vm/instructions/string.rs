use std::{cell::RefCell, rc::Rc};

use crate::{
    ExecutionError,
    vm::{PrimitiveValue, StackValue, Type, TypeError, VM},
};
impl VM {
    fn pop_str(&mut self) -> Result<Rc<RefCell<String>>, ExecutionError> {
        let s = self.stack_pop()?;
        match s {
            StackValue::StringView(view) => Ok(view),
            _ => Err(ExecutionError::TypeMismatch(TypeError {
                expected: Type::Str,
                actual: Type::of(&s),
            })),
        }
    }
    pub fn put_str(&mut self, id: usize) {
        self.stack.push(StackValue::StringView(Rc::new(RefCell::new(
            self.strings[id].clone(),
        ))));
    }
    pub fn str_get(&mut self) -> Result<(), ExecutionError> {
        let value = {
            let index = self.stack_pop()?.int()?;
            let s = self.pop_str()?;
            let l = s.borrow().len();
            let possibly_negative = index % l as isize;
            let possibly_big = possibly_negative as usize + l;
            let idx = possibly_big % l; // see vec_get method for explanation
            s.borrow().as_bytes()[idx] as char
        };
        self.put(PrimitiveValue::Char(value));
        Ok(())
    }
}
