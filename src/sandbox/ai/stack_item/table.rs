use std::cell::RefCell;
use std::collections::BTreeMap;
use std::hash::{Hash, Hasher};
use std::rc::{Rc, Weak};

use crate::sandbox::ai::StackItem as Value;

// ParentTables is the type in the RefCell of TableInterior's parents proporty
pub (super) type ParentTables = Vec<Weak<TableInterior>>;

// TableInterior is the type that the Value::Table has in its Rc
#[derive(Debug)]
pub (in crate::sandbox)struct TableInterior {
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
    /*        pub fn stuff(&mut self, stuffing: Value, key: Value) -> Result<(), ForthErr> {
    //                    match (self, stuffing) {
                        match stuffing {
                            Value::Table(stuffing_rc) => {
                                if self.has_ancester(&stuffing_rc) {
                                     Err(ForthErr::CyclesNotAllowed)
                                } else {
                                    stuffing_rc
                                        .parents
                                        .borrow_mut()
                                        .push(Rc::downgrade(&Rc::new(self)));
                                    self
                                        .map
                                        .borrow_mut()
                                        .insert(key, Value::Table(stuffing_rc));
                                    Ok(())
                                }
                            }
                            _ => {
                                self.map.borrow_mut().insert(key, value);
                                Ok(())
                            }
                        }

            }
    */
}
/*
fn test(){
    let map = RefCell::new(BTreeMap::from([
        (Value::String("child".to_owned()), Value::Int(2))
    ]));
    let parentsss = RefCell::new(Vec::new());
    let table = Value::Table(Rc::new(TableInterior{ map, parents:parentsss }));

    match table{
        Value::Table(x)  if matches!({
            if let TableInterior { map, parents } = x.as_ref() {
                let x = map.borrow().get(&Value::String("Sel".to_owned()));
                None
            } else {
                None
            }
        }, Some(thing)) => {thing;},
        _=> ()
    }
}*/