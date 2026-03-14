use crate::stand_alone_complex::latadog::Relation;

pub trait GetKey<const INDEX: usize, Key> {
    fn get(&self, idx: usize) -> &Key;

}

impl<Key, V> GetKey<0, Key> for Relation<0, Key, (Key, V)> {
    fn get(&self, idx: usize) -> &Key {
        &self.elements[idx].0
    }
}
