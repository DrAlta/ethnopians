use std::{marker::PhantomData, rc::Rc};
#[derive(Debug)]
pub struct Relation<const INDEX: usize, Key, Tuple> {
    pub elements: Rc<Vec<Tuple>>,
    pub index: Vec<usize>,
    pub key: PhantomData<Key>,
}

impl<Key: Eq + Ord, V> From<Rc<Vec<(Key, V)>>> for Relation<0, Key, (Key, V)> {
    fn from(value: Rc<Vec<(Key, V)>>) -> Self {
        let mut index: Vec<usize> = (0..value.len()).collect();
        index.sort_unstable_by(|&a, &b| value[a].0.cmp(&value[b].0));
        index.dedup_by(|a, b| value[*a].0 == value[*b].0);
        Relation {
            elements: value,
            index,
            key: PhantomData,
        }
    }
}
