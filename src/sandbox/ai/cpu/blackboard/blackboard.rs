use std::{borrow::Borrow, collections::BTreeMap, fmt::Debug, hash::Hash};

use super::Variable;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Blackboard<K: Debug + std::cmp::Eq + std::hash::Hash, V: Debug> {
    stack: Vec<BTreeMap<K, Variable<K, V>>>,
}

impl<K: Debug + std::cmp::Eq + std::hash::Hash + Ord, V: Debug> Blackboard<K, V> {
    pub fn insert(&mut self, k: K, v: Variable<K, V>) -> Option<Variable<K, V>> {
        if self.stack.is_empty() {
            self.stack.push(BTreeMap::new());
        };
        self.stack
            .last_mut()
            .expect("we added an items if the stack was empty")
            .insert(k, v)
    }
    pub fn get<Q>(&self, k: &Q) -> Option<&V>
    where
        Q: ?Sized + Hash + Eq + Ord,
        K: Borrow<Q> + Ord,
    {
        let mut current_key = k;
        for i in 1..self.stack.len() + 1 {
            if let Some(hashmap) = self.stack.get(self.stack.len() - i) {
                match hashmap.get(current_key) {
                    Some(Variable::Defer(id)) => current_key = id.borrow(),
                    Some(Variable::Chit(value)) => return Some(value),
                    None => (),
                }
            }
        }
        None
    }
}
impl<K: Debug + Eq + std::hash::Hash, V: Debug> Blackboard<K, V> {
    pub fn from(hash: BTreeMap<K, Variable<K, V>>) -> Self {
        Self { stack: vec![hash] }
    }
    pub fn new() -> Self {
        Self {
            stack: vec![BTreeMap::new()],
        }
    }
    pub fn pop(&mut self) -> Option<BTreeMap<K, Variable<K, V>>> {
        self.stack.pop()
    }
    pub fn push(&mut self, hash: BTreeMap<K, Variable<K, V>>) {
        self.stack.push(hash)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn defer_some_test() {
        let mut blackboard = Blackboard::from(BTreeMap::from([("a", Variable::Chit(1))]));
        blackboard.push(BTreeMap::from([("b", Variable::Defer("a"))]));

        assert_eq!(blackboard.get(&"b"), Some(&1))
    }
    #[test]
    pub fn defer_none_test() {
        let mut blackboard = Blackboard::from(BTreeMap::from([("a", Variable::Chit(1))]));
        blackboard.push(BTreeMap::from([("b", Variable::Defer("c"))]));

        assert_eq!(blackboard.get(&"b"), None)
    }
    #[test]
    pub fn chit_some_test() {
        let mut blackboard = Blackboard::from(BTreeMap::from([("a", Variable::Chit(1))]));
        blackboard.push(BTreeMap::from([("b", Variable::Chit(2))]));

        assert_eq!(blackboard.get(&"b"), Some(&2))
    }
    #[test]
    pub fn chit_none_test() {
        let blackboard = Blackboard::from(BTreeMap::from([("a", Variable::Chit(1))]));
        assert_eq!(blackboard.get(&"b"), None)
    }
}
