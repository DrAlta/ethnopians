use std::fmt::Debug;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Variable<K: Debug, V: Debug> {
    Defer(K),
    Chit(V),
}
impl<K: Debug, V: Debug> Variable<K, V> {
    pub fn defer(value: K) -> Self {
        Self::Defer(value)
    }
}
impl<K: Debug, V: Debug> From<V> for Variable<K, V> {
    fn from(value: V) -> Self {
        Self::Chit(value)
    }
}
