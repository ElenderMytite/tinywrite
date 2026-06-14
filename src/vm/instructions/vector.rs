use crate::vm::{ExecutionError, HeapItem, HeapValue, StackValue, TypeError, VM};
impl HeapItem {
    fn vec_from_vec_item(&mut self) -> Result<&mut Vec<StackValue>, TypeError> {
        match self {
            HeapItem {
                value: HeapValue::Vector(vec),
                ..
            } => Ok(vec),
            _ => Err(TypeError),
        }
    }
}
impl VM {
    pub fn new_vec(&mut self) {
        // eprintln!("allocating new vector");
        self.allocate(HeapItem::new(HeapValue::Vector(Vec::new())));
    }
    fn vec_item_from_heap(&mut self, index: usize) -> Result<&mut HeapItem, TypeError> {
        let heap_val = self.heap.get_mut(&index).unwrap();
        match &mut heap_val.value {
            HeapValue::Vector(_) => Ok(heap_val),
            _ => Err(TypeError),
        }
    }
    pub fn vec_pop(&mut self) -> Result<(), ExecutionError> {
        let pointer = self.stack.pop().unwrap().ptr()?;
        let vec_item = self.vec_item_from_heap(pointer)?;
        let vec = vec_item.vec_from_vec_item()?;
        let value = vec.pop().unwrap_or(StackValue::Nil);
        let _ = value.ptr().inspect(|ptr| {
            self.ref_from_stack(*ptr);
            self.drop_from_heap(pointer, *ptr);
        });
        self.drop_from_stack(pointer, 1);
        self.stack.push(value);
        Ok(())
    }
    pub fn vec_push(&mut self) -> Result<(), ExecutionError> {
        let value = self.stack_pop()?;
        let pointer = self.stack_top()?.ptr()?;
        let vec_item = self.vec_item_from_heap(pointer)?;
        let vec = vec_item.vec_from_vec_item()?;
        println!("pushing {:?} into {}", value, pointer);
        vec.push(value);
        let _ = value.ptr().inspect(|ptr| {
            self.drop_from_stack(*ptr, 1);
            self.ref_from_heap(pointer, *ptr);
        });
        Ok(())
    }
    pub fn vec_get(&mut self) -> Result<(), ExecutionError> {
        let index = self.stack.pop().unwrap().int()? as usize;
        let ptr = self.stack.pop().unwrap().ptr()?;
        let vec_item = self.vec_item_from_heap(ptr)?;
        let vec = vec_item.vec_from_vec_item()?;
        if vec.is_empty() {
            self.stack.push(StackValue::Nil);
        } else {
            let idx = ((index % (vec.len() as isize) as usize + vec.len()) % vec.len()) as usize; // Handle negative indices (python-like)
            let value = vec[idx].clone();
            self.stack.push(value);
        }
        Ok(())
    }
    pub fn len(&mut self) -> Result<(), ExecutionError> {
        let ptr = self.stack_pop()?.ptr()?;
        let structure = &self.heap[&ptr];
        self.stack.push(StackValue::Int(structure.len() as isize));
        self.drop_from_stack(ptr, 1);
        Ok(())
    }
}
