// get can be used for both hmap and vec types in this language. When get command is called,
// vm first checks if it is a hmap or a vec, and then calls corresponding command
use crate::vm::{ExecutionError, HeapValue, VM};
impl VM {
    pub fn get(&mut self) -> Result<(), ExecutionError> {
        let ptr = self.stack.get(self.stack.len() - 2).unwrap().ptr()?; // collection is under the index
        match &*ptr.borrow() {
            HeapValue::HMap(_) => self.hmap_get()?,
            HeapValue::Vector(_) => self.vec_get()?,
        }
        Ok(())
    }
}
