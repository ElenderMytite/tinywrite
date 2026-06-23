use std::{cell::RefCell, collections::HashMap, fmt::Display, rc::Rc};

use crate::{ExecutionError, vm::HeapValue::HMap};

#[derive(Debug, Clone, Copy)]
pub struct TypeError {
    pub expected: Type,
    pub actual: Type,
}
impl std::fmt::Display for TypeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Value expected to be {} but is {}!",
            self.expected, self.actual
        )
    }
}
#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub enum HashableValue {
    Primitive(PrimitiveValue),
    Pair(Box<HashableValue>, Box<HashableValue>),
    StringView(String),
}
#[derive(Debug, Clone)]
pub enum HeapValue {
    Vector(Vec<StackValue>),
    HMap(HashMap<HashableValue, StackValue>),
}
impl HeapValue {
    pub fn len(&self) -> usize {
        match self {
            HeapValue::Vector(l) => l.len(),
            HMap(l) => l.len(),
        }
    }
    pub fn pop_vec(&mut self) -> Result<StackValue, ExecutionError> {
        match self {
            HeapValue::Vector(vec) => match vec.pop() {
                Some(value) => Ok(value),
                None => Err(ExecutionError::PopFromEmpty),
            },
            heap_val => Err(ExecutionError::TypeMismatch(TypeError {
                expected: Type::List,
                actual: Type::of_heap_val(heap_val),
            })),
        }
    }
    pub fn push_vec(&mut self, value: StackValue) -> Result<(), ExecutionError> {
        match self {
            HeapValue::Vector(vec) => Ok(vec.push(value)),
            heap_val => Err(ExecutionError::TypeMismatch(TypeError {
                expected: Type::List,
                actual: Type::of_heap_val(heap_val),
            })),
        }
    }
}
#[derive(Debug, Clone)]
pub enum StackValue {
    StringView(Rc<RefCell<String>>),
    Primitive(PrimitiveValue),
    Pointer(Rc<RefCell<HeapValue>>),
    Pair(Rc<RefCell<StackValue>>, Rc<RefCell<StackValue>>),
}
impl Default for StackValue {
    fn default() -> Self {
        Self::Primitive(PrimitiveValue::Void)
    }
}
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum PrimitiveValue {
    Bool(bool),
    Int(isize),
    Char(char),
    Type(Type),
    Void,
}
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum Type {
    Type,
    Bool,
    Void,
    Int,
    Char,
    List,
    HMap,
    Str,
    Pair,
}
impl Display for Type {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Type::Type => "type",
                Type::Void => "void",
                Type::Bool => "boolean",
                Type::Int => "integer",
                Type::Char => "character",
                Type::List => "vector",
                Type::HMap => "hashmap",
                Type::Str => "text",
                Type::Pair => "pair",
            },
        )
    }
}
impl Type {
    pub fn of(value: &StackValue) -> Self {
        match value {
            StackValue::Primitive(PrimitiveValue::Bool(_)) => Self::Bool,
            StackValue::Primitive(PrimitiveValue::Int(_)) => Self::Int,
            StackValue::Primitive(PrimitiveValue::Char(_)) => Self::Char,
            StackValue::Primitive(PrimitiveValue::Void) => Self::Void,
            StackValue::Primitive(PrimitiveValue::Type(_)) => Self::Type,
            StackValue::Pair(_, _) => Self::Pair,
            StackValue::Pointer(value) => match &*value.borrow() {
                HeapValue::HMap(_) => Self::HMap,
                HeapValue::Vector(_) => Self::List,
            },
            StackValue::StringView(_) => Type::Str,
        }
    }
    pub fn of_heap_val(value: &HeapValue) -> Self {
        match value {
            HeapValue::Vector(_) => Self::List,
            HeapValue::HMap(_) => Self::HMap,
        }
    }
}
impl StackValue {
    pub fn primitive(&self) -> Option<&PrimitiveValue> {
        match self {
            StackValue::Primitive(primitive) => Some(primitive),
            _ => None,
        }
    }
    pub fn is_hashable(&self) -> bool {
        match self {
            StackValue::Primitive(_) => true,
            StackValue::Pointer(r) => Type::of_heap_val(&*r.borrow()) == Type::Str,
            StackValue::Pair(head, tail) => {
                head.borrow().is_hashable() && tail.borrow().is_hashable()
            }
            StackValue::StringView(_) => true,
        }
    }
    pub fn hashable(self) -> Option<HashableValue> {
        if !self.is_hashable() {
            return None;
        }
        match self {
            StackValue::Primitive(primitive) => Some(HashableValue::Primitive(primitive)),
            Self::StringView(s) => Some(HashableValue::StringView(s.borrow().clone())),
            _ => unreachable!(),
        }
    }
    pub fn int(&self) -> Result<isize, TypeError> {
        match self {
            Self::Primitive(PrimitiveValue::Int(x)) => Ok(*x),
            _ => Err(TypeError {
                expected: Type::Int,
                actual: Type::of(self),
            }),
        }
    }
    pub fn bool(&self) -> Result<bool, TypeError> {
        match self {
            Self::Primitive(PrimitiveValue::Bool(b)) => Ok(*b),
            _ => Err(TypeError {
                expected: Type::Bool,
                actual: Type::of(self),
            }),
        }
    }
    pub fn char(&self) -> Result<char, TypeError> {
        match self {
            Self::Primitive(PrimitiveValue::Char(c)) => Ok(*c),
            _ => Err(TypeError {
                expected: Type::Char,
                actual: Type::of(self),
            }),
        }
    }
    pub fn ptr(&self) -> Result<Rc<RefCell<HeapValue>>, ExecutionError> {
        match self {
            Self::Pointer(ptr) => Ok(Rc::clone(ptr)),
            _ => Err(ExecutionError::NonPointerValue),
        }
    }
}
pub fn format_value(value: &PrimitiveValue) -> String {
    match value {
        PrimitiveValue::Type(value_type) => format!("'{}", value_type),
        PrimitiveValue::Void => String::from("<void>"),
        PrimitiveValue::Int(x) => x.to_string(),
        PrimitiveValue::Bool(b) => b.to_string(),
        PrimitiveValue::Char(c) => c.to_string(),
    }
}
pub fn format_structure(structure: &HeapValue) -> String {
    match structure {
        HeapValue::Vector(values) => format!("{values:?}"),
        HeapValue::HMap(map) => format!("{map:?}"),
    }
}
