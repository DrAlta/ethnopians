use std::collections::BTreeMap;
use std::hash::Hash;

use qol::logy;

use crate::ai::forth::StackItem as Value;

// TableInterior is the type that the Value::Table has in its Arc
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct TableInterior<EntityId> {
    //map holds the key, value pairs of the table
    pub map: BTreeMap<Value<EntityId>, Value<EntityId>>,
}
impl<EntityId> TableInterior<EntityId> {
    pub fn new() -> Self {
        Self {
            map: BTreeMap::new(),
        }
    }
}
impl<EntityId: Ord> TableInterior<EntityId> {
    pub fn insert(&mut self, k: Value<EntityId>, v: Value<EntityId>) -> Option<Value<EntityId>> {
        self.map.insert(k, v)
    }
    pub fn get(&mut self, k: &Value<EntityId>) -> Option<&Value<EntityId>> {
        self.map.get(k)
    }
}

pub trait TableGet<T, EntityId> {
    fn table_get(&self, k: T) -> Option<&Value<EntityId>>;
}

impl<EntityId: Ord> TableGet<&Value<EntityId>, EntityId>
    for BTreeMap<Value<EntityId>, Value<EntityId>>
{
    fn table_get(&self, k: &Value<EntityId>) -> Option<&Value<EntityId>> {
        self.get(k)
    }
}
impl<EntityId: Ord + std::fmt::Debug, T: Into<Value<EntityId>> + std::fmt::Debug>
    TableGet<T, EntityId> for BTreeMap<Value<EntityId>, Value<EntityId>>
{
    fn table_get(&self, k: T) -> Option<&Value<EntityId>> {
        logy!("debug", "{k:?}");
        let k2: Value<EntityId> = k.into();
        logy!("debug", "{k2:?}");
        self.get(&k2)
    }
}
