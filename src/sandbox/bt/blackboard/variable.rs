use std::fmt::Debug;

#[derive(Debug)]
pub enum Variable<K: Debug, V: Debug>{
    Defer(K),
    Chit(V),
}
