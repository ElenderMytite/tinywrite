use crate::vm::{HeapItem, StackValue};

use super::VM;
impl VM {
    pub(super) fn allocate(&mut self, item: HeapItem) {
        self.heap.insert(self.next_addr, item);
        self.ref_from_stack(self.next_addr);
        self.stack.push(StackValue::Pointer(self.next_addr));
        self.next_addr += 1;
    }
    fn reference_heap(&mut self, addr: usize) {
        eprintln!("adding reference to {addr}...",);
        self.heap.get_mut(&addr).unwrap().refs += 1;
    }
    /// reduces amount of references to the value, if all references cleared, removes the value and all references it had
    fn free(&mut self, addr: usize, amount: usize) {
        let val = &mut self.heap.get_mut(&addr).unwrap();
        val.refs -= amount;
        if val.refs <= 0 {
            for (reference, count) in val.self_refs.clone().into_iter() {
                self.free(reference, count);
            }
            self.heap.remove(&addr);
        }
    }
    pub(super) fn drop_from_stack(&mut self, addr: usize, amount: usize) {
        self.stack_refs
            .entry(addr)
            .and_modify(|count| *count -= amount)
            .or_default();
        if self.stack_refs[&addr] <= 0 {
            self.stack_refs.remove(&addr);
        }
        self.free(addr, amount);
    }
    pub(super) fn drop_from_heap(&mut self, from: usize, to: usize) {
        self.heap
            .get_mut(&from)
            .unwrap()
            .self_refs
            .entry(to)
            .and_modify(|x| *x -= 1);
        self.free(to, 1);
    }
    pub(super) fn drop_from_vars(&mut self, addr: usize) {
        self.vars_refs
            .entry(addr)
            .and_modify(|count| *count -= 1)
            .or_default();
        if self.vars_refs[&addr] <= 0 {
            self.vars_refs.remove(&addr);
        }
        self.free(addr, 1);
    }
    pub(super) fn ref_from_heap(&mut self, from: usize, to: usize) {
        self.heap
            .get_mut(&from)
            .unwrap()
            .self_refs
            .entry(to)
            .and_modify(|x| *x += 1)
            .or_insert(1);
        self.reference_heap(to);
    }
    pub(super) fn ref_from_vars(&mut self, addr: usize) {
        self.vars_refs
            .entry(addr)
            .and_modify(|count| *count += 1)
            .or_insert(1);
        self.reference_heap(addr);
    }
    pub(super) fn ref_from_stack(&mut self, addr: usize) {
        self.stack_refs
            .entry(addr)
            .and_modify(|count| *count += 1)
            .or_insert(1);
        self.reference_heap(addr);
    }
}
