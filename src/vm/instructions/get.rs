// get can be used for both hmap and vec types in this language. When get command is called,
// vm first checks if it is a hmap or a vec, and then calls corresponding command
use crate::vm::{HeapValue, TypeError, VM};
impl VM {
    fn extract_heap_value(&mut self, index: usize) -> Result<&HeapValue, TypeError> {
        Ok(&mut self.heap[index])
    }
    pub fn get(&mut self, index_size: usize) -> Result<(), TypeError> {
        let ptr = self
            .stack
            .get(self.stack.len() - 1 - index_size)
            .unwrap()
            .ptr()?; // collection is under the index
        let collection = self.extract_heap_value(ptr)?;
        match collection {
            HeapValue::HMap(_) => self.hmap_get()?,
            HeapValue::Vector(_) => self.vec_get()?,
            _ => todo!(),
        }
        Ok(())
    }
}
