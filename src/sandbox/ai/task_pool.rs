use std::{borrow::Borrow, collections::BTreeMap};

use crate::sandbox::ai::{Thread, ThreadName, Instruction};

#[derive(Debug,Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct TaskPool(BTreeMap<ThreadName, Thread>);
impl TaskPool {
    pub fn new() -> Self {
        Self(BTreeMap::new())
    }
    pub fn core() -> Self {
        Self(BTreeMap::from([(
            "combine@i".to_owned(), Thread::from([
                Instruction::ForthFindInInventory,
                Instruction::ForthSomeEntityId,
                Instruction::ForthIf(6),
                Instruction::ForthSwap,
                Instruction::ForthFindInInventory,
                Instruction::ForthSomeEntityId,
                Instruction::ForthIf(2),
                Instruction::ForthSwap,
                Instruction::ForthAction(super::InpulseId::UseOn),
                Instruction::ForthReturn
            ])
            )]))
    }
    pub fn get<Q: ?Sized + Ord>(&self, k: &Q) -> Option<&Thread>
    where ThreadName: Borrow<Q> {
        self.0.get(k)
    }
    pub fn insert(&mut self, key: ThreadName, value:Thread) -> Option<Thread> {
        self.0.insert(key, value)
    }
    pub fn contains_key<Q: ?Sized + Ord>(&self, key: &Q) -> bool
    where ThreadName: Borrow<Q> {
        self.0.contains_key(key)
    }
    pub fn extend<I: IntoIterator<Item = (ThreadName, Thread)>>(&mut self, iter:I) {
        self.0.extend(iter);
    }

}

impl IntoIterator for TaskPool {
    type Item = (ThreadName, Thread); // The items produced by the iterator
    type IntoIter = std::collections::btree_map::IntoIter<ThreadName, Thread>; // The iterator type

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter() // Delegates to the BTreeMap's into_iter
    }
}


impl<'a> IntoIterator for &'a TaskPool {
    type Item = (&'a ThreadName, &'a Thread); // The iterator produces references
    type IntoIter = std::collections::btree_map::Iter<'a, ThreadName, Thread>; // The iterator type

    fn into_iter(self) -> Self::IntoIter {
        self.0.iter() // Delegates to the BTreeMap's iter method
    }
}

impl<T: Into<BTreeMap<ThreadName, Thread>>> From<T> for TaskPool {
    fn from(value: T) -> Self {
        Self(value.into())
    }
}
