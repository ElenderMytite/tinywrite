use std::io::{Write, stdout};

use crate::{
    ExecutionError,
    vm::{HashableValue, HeapValue, PrimitiveValue, StackValue, VM, format_value},
};

impl VM {
    fn dump_hashable_value(&mut self, value: &HashableValue) {
        match value {
            HashableValue::Primitive(p) => self.dump_stack_value(&StackValue::Primitive(*p)),
            HashableValue::Str(s) => {
                print!("{s}");
                stdout().flush().unwrap();
                return;
            }
            HashableValue::Pair(head, tail) => {
                print!("(");
                self.dump_hashable_value(head);
                print!(" ");
                self.dump_hashable_value(tail);
                print!(")");
            }
        }
    }
    fn dump_stack_value(&mut self, value: &StackValue) {
        if let StackValue::StringView(s) = value {
            print!("{string}", string = s.borrow());
            stdout().flush().unwrap();
            return;
        }
        let primitive = value.primitive().copied();
        if let Some(PrimitiveValue::Char('\n')) = primitive {
            println!();
            self.flush = false;
        } else if let Some(prim) = primitive {
            print!("{}", format_value(&prim));
            self.flush = true;
        } else if let Ok(structure) = value.ptr() {
            match &*structure.borrow() {
                HeapValue::HMap(map) => {
                    print!("{{");
                    for (k, v) in map.iter() {
                        self.dump_hashable_value(k);
                        print!(": ");
                        self.dump_stack_value(v);
                        print!(", ")
                    }
                    print!("}}");
                }
                HeapValue::Vector(arr) => {
                    print!("[");
                    for v in arr {
                        self.dump_stack_value(v);
                        print!(", ")
                    }
                    print!("]");
                }
            }
        } else {
            print!("<unprintable value>");
            self.flush = true;
        }
    }
    pub fn dump(&mut self) -> Result<(), ExecutionError> {
        let value = self.stack_pop()?;
        self.dump_stack_value(&value);
        Ok(())
    }
}
