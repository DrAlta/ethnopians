use std::{borrow::Borrow, collections::BTreeMap};

use crate::ai::forth::{Thread, ThreadId};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ThreadPool<InpulseId, EntityId>(BTreeMap<ThreadId, Thread<InpulseId, EntityId>>);
impl<InpulseId, EntityId> ThreadPool<InpulseId, EntityId> {
    pub fn new() -> Self {
        Self(BTreeMap::new())
    }
    pub fn get<Q: ?Sized + Ord>(&self, k: &Q) -> Option<&Thread<InpulseId, EntityId>>
    where
        ThreadId: Borrow<Q>,
    {
        self.0.get(k)
    }
    pub fn insert(&mut self, key: ThreadId, value: Thread<InpulseId, EntityId>) -> Option<Thread<InpulseId, EntityId>> {
        self.0.insert(key, value)
    }
    pub fn contains_key<Q: ?Sized + Ord>(&self, key: &Q) -> bool
    where
        ThreadId: Borrow<Q>,
    {
        self.0.contains_key(key)
    }
    pub fn extend<I: IntoIterator<Item = (ThreadId, Thread<InpulseId, EntityId>)>>(&mut self, iter: I) {
        self.0.extend(iter);
    }
}

impl<InpulseId, EntityId> IntoIterator for ThreadPool<InpulseId, EntityId> {
    type Item = (ThreadId, Thread<InpulseId, EntityId>); // The items produced by the iterator
    type IntoIter = std::collections::btree_map::IntoIter<ThreadId, Thread<InpulseId, EntityId>>; // The iterator type

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter() // Delegates to the BTreeMap's into_iter
    }
}

impl<'a, InpulseId, EntityId> IntoIterator for &'a ThreadPool<InpulseId, EntityId> {
    type Item = (&'a ThreadId, &'a Thread<InpulseId, EntityId>); // The iterator produces references
    type IntoIter = std::collections::btree_map::Iter<'a, ThreadId, Thread<InpulseId, EntityId>>; // The iterator type

    fn into_iter(self) -> Self::IntoIter {
        self.0.iter() // Delegates to the BTreeMap's iter method
    }
}

impl<InpulseId, EntityId, T: Into<BTreeMap<ThreadId, Thread<InpulseId, EntityId>>>> From<T> for ThreadPool<InpulseId, EntityId> {
    fn from(value: T) -> Self {
        Self(value.into())
    }
}
