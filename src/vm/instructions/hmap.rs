use crate::vm::{ExecutionError, HeapItem, HeapValue, StackValue, TypeError, VM};
use std::collections::HashMap;
impl VM {
    fn extract_hmap_item(&mut self, index: usize) -> Result<&mut HeapItem, TypeError> {
        let val = self.heap.get_mut(&index).unwrap();
        match val {
            HeapItem {
                value: HeapValue::HMap(_),
                ..
            } => Ok(val),
            _ => Err(TypeError),
        }
    }
    pub fn new_hmap(&mut self) {
        self.allocate(HeapItem::new(HeapValue::HMap(HashMap::new())));
    }
    pub fn hmap_insert(&mut self) -> Result<(), ExecutionError> {
        let value = self.stack_pop()?;
        let key = self.stack_pop()?;
        let pointer = self.stack_pop()?.ptr()?;
        {
            let hmap_item = self.extract_hmap_item(pointer)?;
            match hmap_item {
                HeapItem {
                    value: HeapValue::HMap(map),
                    ..
                } => map.insert(key, value),
                _ => unreachable!(),
            };
        }
        // eprintln!("Successfully extracted hmap: {:?}", hmap.clone());
        // dbg!(key, value, ptr, hmap.clone());
        let _ = value.ptr().inspect(|ptr| {
            self.ref_from_heap(pointer, *ptr);
        });
        let _ = key.ptr().inspect(|ptr| {
            self.ref_from_heap(pointer, *ptr);
        });
        self.drop_from_stack(pointer, 1);
        Ok(())
    }
    pub fn hmap_get(&mut self) -> Result<(), ExecutionError> {
        let key = self.stack_pop()?;
        let pointer = self.stack.pop().unwrap().ptr()?;
        let value = {
            let hmap_item = self.extract_hmap_item(pointer)?;
            let hmap = match hmap_item {
                HeapItem {
                    value: HeapValue::HMap(map),
                    ..
                } => map,
                _ => unreachable!(),
            };
            *hmap.get(&key).unwrap_or(&StackValue::Nil) // TODO: implement option type
        };
        // if value was a pointer, increase refence count
        let _ = value.ptr().inspect(|ptr| {
            self.ref_from_stack(*ptr); // value was copied, not moved
        });
        self.stack.push(value);
        self.drop_from_stack(pointer, 1);
        Ok(())
    }
    pub fn hmap_contains(&mut self) -> Result<(), ExecutionError> {
        let key = self.stack_pop()?;
        let ptr = self.stack.pop().unwrap().ptr()?;
        let hmap_item = self.extract_hmap_item(ptr)?;
        let value = {
            match hmap_item {
                HeapItem {
                    value: HeapValue::HMap(map),
                    ..
                } => map.contains_key(&key),
                _ => unreachable!(),
            }
        };
        let _ = key.ptr().inspect(|ptr| {
            if value {
                self.drop_from_stack(*ptr, 1);
            }
        });
        self.stack.push(StackValue::Bool(value));
        Ok(())
    }
    pub fn hmap_remove(&mut self) -> Result<(), ExecutionError> {
        // what needs to be tracked for reference counting: key in map, key on stack, removed
        let key = self.stack_pop()?;
        let pointer = self.stack_pop()?.ptr()?;
        let value = {
            let hmap_item = self.extract_hmap_item(pointer)?;
            match hmap_item {
                HeapItem {
                    value: HeapValue::HMap(map),
                    ..
                } => map.remove(&key),
                _ => unreachable!(),
            }
        };
        self.drop_from_stack(pointer, 1);
        //key is already put in the hmap, so after popping it from stack amount of references decreases
        let _ = key.ptr().inspect(|ptr| {
            self.drop_from_stack(*ptr, 1);
        });
        // if value has actually been removed, drop references of the key and value
        if value.is_some() {
            let _ = key.ptr().inspect(|ptr| self.drop_from_heap(pointer, *ptr));
            let _ = value
                .unwrap()
                .ptr()
                .inspect(|ptr| self.drop_from_heap(pointer, *ptr));
        }
        // value has just been removed from hmap, but put onto stack
        self.stack.push(value.unwrap_or(StackValue::Nil)); // TODO: implement option type

        Ok(())
    }
}
