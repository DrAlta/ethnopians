use std::cell::RefCell;
use std::collections::BTreeMap;
use std::hash::{Hash, Hasher};
use std::rc::{Rc, Weak};

use crate::sandbox::ai::StackItem as Value;

// ParentTables is the type in the RefCell of TableInterior's parents proporty
pub(super) type ParentTables = Vec<Weak<TableInterior>>;

// TableInterior is the type that the Value::Table has in its Rc
#[derive(Debug)]
pub struct TableInterior {
    //map holds the key, value pairs of the table
    pub map: RefCell<BTreeMap<Value, Value>>,
    //parents holds the list of TableInteriors that hold this table.
    pub parents: RefCell<ParentTables>,
}
impl Hash for TableInterior {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.map.borrow().hash(state);
    }
}

impl PartialOrd for TableInterior {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for TableInterior {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.map.cmp(&other.map)
    }
}
impl PartialEq for TableInterior {
    fn eq(&self, other: &Self) -> bool {
        self.map.eq(&other.map)
    }
}
impl Eq for TableInterior {}
impl TableInterior {
    pub fn has_ancester(&self, other: &Rc<TableInterior>) -> bool {
        self.parents.borrow().iter().any(|x| match x.upgrade() {
            None => false,
            Some(rc) => {
                if Rc::ptr_eq(&rc, other) {
                    true
                } else {
                    rc.has_ancester(other)
                }
            }
        })
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
impl<T: Into<Value>> TableGet<T> for BTreeMap<Value, Value> {
    fn table_get(&self, k: T) -> Option<&Value> {
        let k2: Value = k.into();
        self.get(&k2)
    }
}
