use std::{cell::RefCell, rc::Rc};

use crate::{
    ExecutionError::TypeMismatch,
    vm::{ExecutionError, HeapValue::Vector, PrimitiveValue, StackValue, Type, TypeError, VM},
};
impl VM {
    pub fn vec_new(&mut self) {
        self.stack
            .push(StackValue::Pointer(Rc::new(RefCell::new(Vector(
                Vec::new(),
            )))));
    }
    pub fn vec_pop(&mut self) -> Result<(), ExecutionError> {
        let vector = self.stack_pop()?;
        let actual_type_of_vector = Type::of(&vector);
        match vector {
            StackValue::Pointer(pointer) => Ok(self.stack.push((pointer.borrow_mut()).pop_vec()?)),
            _ => Err(TypeMismatch(TypeError {
                expected: Type::List,
                actual: actual_type_of_vector,
            })),
        }
    }
    pub fn vec_push(&mut self) -> Result<(), ExecutionError> {
        let value = self.stack_pop()?;
        let vector = self.stack_pop()?;
        let actual = Type::of(&vector);
        match vector {
            StackValue::Pointer(pointer) => (&mut *pointer.borrow_mut()).push_vec(value),
            _ => Err(TypeMismatch(TypeError {
                expected: Type::List,
                actual,
            })),
        }
    }
    pub fn vec_get(&mut self) -> Result<(), ExecutionError> {
        let index = self.stack_pop()?.int()? as usize;
        {
            let ptr = self.stack_pop()?.ptr()?;
            match &*ptr.borrow() {
                Vector(vector) => {
                    if vector.is_empty() {
                        self.stack.push(StackValue::Primitive(PrimitiveValue::Void));
                    } else {
                        let idx = ((index % (vector.len() as isize) // get value in range (-vec.len();vec.len())
                    as usize + vector.len()) // add vec.len() so value is positive
                    % vector.len()) as usize; // go to range [0, vec.len() - 1]
                        let value = vector[idx].clone();
                        self.stack.push(value);
                    }
                }
                value => {
                    return Err(ExecutionError::TypeMismatch(TypeError {
                        expected: Type::List,
                        actual: Type::of_heap_val(&value),
                    }));
                }
            }
        };
        Ok(())
    }
    pub fn len(&mut self) -> Result<(), ExecutionError> {
        let ptr = self.stack_pop()?.ptr()?;
        let structure = &*ptr.borrow_mut();
        self.stack.push(StackValue::Primitive(PrimitiveValue::Int(
            structure.len() as isize
        )));
        Ok(())
    }
}
