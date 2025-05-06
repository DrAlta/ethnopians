use std::fmt::Debug;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Variable<K: Debug, V: Debug> {
    Defer(K),
    Chit(V),
}
