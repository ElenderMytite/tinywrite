use std::{cell::RefCell, collections::HashMap, rc::Rc};

use crate::{
    ExecutionError::{self, TypeMismatch},
    vm::{HashableValue, HeapValue, PrimitiveValue, Type, TypeError, VM},
};

impl VM {
    pub fn hmap_new(&mut self) {
        self.stack
            .push(crate::vm::StackValue::Pointer(Rc::new(RefCell::new(
                HeapValue::HMap(HashMap::new()),
            ))));
    }
    pub fn hmap_insert(&mut self) -> Result<(), ExecutionError> {
        let value = self.stack_pop()?;
        let key = { self.pop_key()? };
        let collection = self.stack_top()?.ptr()?;
        match &mut *collection.borrow_mut() {
            HeapValue::HMap(hash_map) => {
                hash_map.insert(key, value);
                Ok(())
            }
            c => Err(ExecutionError::TypeMismatch(TypeError {
                expected: Type::HMap,
                actual: Type::of_heap_val(c),
            })),
        }
    }
    pub fn hmap_contains(&mut self) -> Result<(), ExecutionError> {
        let key = self.pop_key()?;
        let ptr = self.pop_hmap()?;
        let value = match &*ptr.borrow() {
            HeapValue::HMap(map) => map.contains_key(&key),
            _ => unreachable!(),
        };
        self.put(PrimitiveValue::Bool(value));
        Ok(())
    }
    pub fn hmap_remove(&mut self) -> Result<(), ExecutionError> {
        let key = self.pop_key()?;
        let hmap = self.pop_hmap()?;
        match &mut *hmap.borrow_mut() {
            HeapValue::HMap(map) => {
                map.remove(&key);
            }
            _ => unreachable!(),
        }
        Ok(())
    }
    fn pop_key<'b>(&'b mut self) -> Result<HashableValue, ExecutionError> {
        let key = self.stack_pop()?;
        if key.is_hashable() {
            Ok(key.hashable().unwrap())
        } else {
            Err(ExecutionError::NonHashableValue)
        }
    }
    fn pop_hmap(&mut self) -> Result<Rc<RefCell<HeapValue>>, ExecutionError> {
        let collection = self.stack_pop()?.ptr()?;
        let is_hmap = matches!(&*collection.borrow(), HeapValue::HMap(_));
        if is_hmap {
            Ok(collection.clone())
        } else {
            Err(TypeMismatch(TypeError {
                expected: Type::HMap,
                actual: Type::of_heap_val(&*collection.borrow()),
            }))
        }
    }
    pub fn hmap_get(&mut self) -> Result<(), ExecutionError> {
        let key = self.pop_key()?;
        let collection = self.pop_hmap()?;
        let value = match &*collection.borrow() {
            HeapValue::HMap(map) => map.get(&key).cloned().ok_or(ExecutionError::NonExistingKey),
            _ => unreachable!(),
        };
        self.stack.push(value?);
        Ok(())
    }
}
