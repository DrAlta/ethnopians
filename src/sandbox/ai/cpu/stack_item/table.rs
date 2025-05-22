use std::collections::BTreeMap;
use std::hash::Hash;

use qol::logy;

use crate::sandbox::ai::StackItem as Value;

// TableInterior is the type that the Value::Table has in its Arc
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct TableInterior {
    //map holds the key, value pairs of the table
    pub map: BTreeMap<Value, Value>,
}
impl TableInterior {
    pub fn new() -> Self {
        Self {
            map: BTreeMap::new(),
        }
    }
    pub fn insert(&mut self, k: Value, v: Value) -> Option<Value> {
        self.map.insert(k, v)
    }
    pub fn get(&mut self, k: &Value) -> Option<&Value> {
        self.map.get(k)
    }
}

pub trait TableGet<T> {
    fn table_get(&self, k: T) -> Option<&Value>;
}

impl TableGet<&Value> for BTreeMap<Value, Value> {
    fn table_get(&self, k: &Value) -> Option<&Value> {
        self.get(k)
    }
}
impl<T: Into<Value> + std::fmt::Debug> TableGet<T> for BTreeMap<Value, Value> {
    fn table_get(&self, k: T) -> Option<&Value> {
        logy!("debug", "{k:?}");
        let k2: Value = k.into();
        logy!("debug", "{k2:?}");
        self.get(&k2)
    }
}
