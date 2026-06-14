use std::collections::HashMap;

use crate::vm::VM;

#[derive(Debug, Clone, Copy)]
pub struct TypeError;
impl std::fmt::Display for TypeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Value popped from the self.stack has an unexpected type!"
        )
    }
}
#[derive(Clone)]
pub struct HeapItem {
    pub(super) value: HeapValue,
    pub(super) self_refs: HashMap<usize, usize>,
    pub(super) refs: usize,
}
impl std::fmt::Debug for HeapItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{:?} self_refs: {:?}, refs: {}",
            self.value, self.self_refs, self.refs
        )
    }
}
#[derive(Debug, Clone)]
pub(super) enum HeapValue {
    Vector(Vec<StackValue>),
    HMap(HashMap<StackValue, StackValue>),
    _Str(String),
    Group(StackValue, StackValue),
}
impl HeapItem {
    pub(super) fn len(&self) -> usize {
        match &self.value {
            HeapValue::Vector(vec) => vec.len(),
            HeapValue::HMap(hmap) => hmap.len(),
            HeapValue::_Str(s) => s.len(),
            HeapValue::Group(..) => 2,
        }
    }
    pub(super) fn new(value: HeapValue) -> Self {
        Self {
            value,
            refs: 0,
            self_refs: HashMap::new(),
        }
    }
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum StackValue {
    Bool(bool),
    Int(isize),
    Char(char),
    Pointer(usize), // index in the self.heap
    Nil,
}
#[derive(Debug, Clone, Copy)]
pub enum Type {
    Bool,
    Int,
    Char,
    Pointer,
    Vector,
    HMap,
    String,
    Group,
}
impl StackValue {
    pub fn int(&self) -> Result<isize, TypeError> {
        match &self {
            Self::Int(x) => Ok(*x),
            _ => Err(TypeError),
        }
    }
    pub fn bool(&self) -> Result<bool, TypeError> {
        match &self {
            Self::Bool(b) => Ok(*b),
            _ => Err(TypeError),
        }
    }
    pub fn char(&self) -> Result<char, TypeError> {
        match &self {
            Self::Char(c) => Ok(*c),
            _ => Err(TypeError),
        }
    }
    pub fn ptr(&self) -> Result<usize, TypeError> {
        match &self {
            Self::Pointer(p) => Ok(*p),
            _ => Err(TypeError),
        }
    }
}
pub fn format_value(value: &StackValue, vm: &VM) -> String {
    match value {
        StackValue::Nil => String::from("_"),
        StackValue::Int(x) => x.to_string(),
        StackValue::Bool(b) => b.to_string(),
        StackValue::Char(c) => c.to_string(),
        StackValue::Pointer(p) => {
            let heap_val = &vm
                .heap
                .get(p)
                .unwrap_or_else(|| panic!("pointers must be valid"))
                .value;
            match heap_val {
                HeapValue::Vector(vec) => {
                    let elements: Vec<String> = vec.iter().map(|v| format_value(v, vm)).collect();
                    format!("[{}]", elements.join(", "))
                }
                HeapValue::HMap(map) => {
                    let elements: Vec<String> = map
                        .iter()
                        .map(|(k, v)| format!("{}: {}", format_value(k, vm), format_value(v, vm)))
                        .collect();
                    format!("{{{}}}", elements.join(", "))
                }
                HeapValue::_Str(s) => s.clone(),
                HeapValue::Group(first, second) => {
                    format!(
                        "({}, {})",
                        format_value(first, vm),
                        format_value(second, vm)
                    )
                }
            }
        }
    }
}
