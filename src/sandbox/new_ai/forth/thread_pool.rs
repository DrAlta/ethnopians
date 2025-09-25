use std::{borrow::Borrow, collections::BTreeMap};

use crate::sandbox::new_ai::forth::{Thread, ThreadId};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ThreadPool(BTreeMap<ThreadId, Thread>);
impl ThreadPool {
    pub fn new() -> Self {
        Self(BTreeMap::new())
    }
    pub fn get<Q: ?Sized + Ord>(&self, k: &Q) -> Option<&Thread>
    where
        ThreadId: Borrow<Q>,
    {
        self.0.get(k)
    }
    pub fn insert(&mut self, key: ThreadId, value: Thread) -> Option<Thread> {
        self.0.insert(key, value)
    }
    pub fn contains_key<Q: ?Sized + Ord>(&self, key: &Q) -> bool
    where
        ThreadId: Borrow<Q>,
    {
        self.0.contains_key(key)
    }
    pub fn extend<I: IntoIterator<Item = (ThreadId, Thread)>>(&mut self, iter: I) {
        self.0.extend(iter);
    }
}

impl IntoIterator for ThreadPool {
    type Item = (ThreadId, Thread); // The items produced by the iterator
    type IntoIter = std::collections::btree_map::IntoIter<ThreadId, Thread>; // The iterator type

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter() // Delegates to the BTreeMap's into_iter
    }
}

impl<'a> IntoIterator for &'a ThreadPool {
    type Item = (&'a ThreadId, &'a Thread); // The iterator produces references
    type IntoIter = std::collections::btree_map::Iter<'a, ThreadId, Thread>; // The iterator type

    fn into_iter(self) -> Self::IntoIter {
        self.0.iter() // Delegates to the BTreeMap's iter method
    }
}

impl<T: Into<BTreeMap<ThreadId, Thread>>> From<T> for ThreadPool {
    fn from(value: T) -> Self {
        Self(value.into())
    }
}
