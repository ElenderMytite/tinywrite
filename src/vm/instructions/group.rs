use crate::{
    ExecutionError,
    vm::{HeapItem, HeapValue, StackValue, TypeError, VM},
};
impl VM {
    fn new_group(&mut self, a: StackValue, b: StackValue) {
        self.allocate(HeapItem::new(HeapValue::Group(a, b)));
        match a {
            StackValue::Pointer(ptr) => self.ref_from_heap(self.next_addr - 1, ptr),
            _ => (),
        }
        match b {
            StackValue::Pointer(ptr) => self.ref_from_heap(self.next_addr - 1, ptr),
            _ => (),
        }
    }
    fn ensure_group(&mut self, addr: usize) -> Result<(), ExecutionError> {
        match self.heap[&addr].value {
            HeapValue::Group(..) => Ok(()),
            _ => Err(ExecutionError::TypeMismatch(TypeError)),
        }
    }
    pub fn prepend(&mut self) -> Result<(), ExecutionError> {
        let value = self.stack_pop()?;
        let addr = self.stack_pop()?.ptr()?;
        self.ensure_group(addr)?;
        self.new_group(value, StackValue::Pointer(addr));
        Ok(())
    }
    pub fn group(&mut self) -> Result<(), ExecutionError> {
        let b = self.stack_pop()?;
        let a = self.stack_pop()?;
        self.new_group(a, b);
        Ok(())
    }
}
