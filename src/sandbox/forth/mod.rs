//! todo:
//! you have `chit`s that can hold a `Value` you can't put a chit on the stack only referances to the chit
//!
//! can tables have chits, can thet hold any value of only refs to chits
//!
//! the question is do we want to be able to make another Table point to the same chit a
//!
//! let a = Chit(1)
//! let b = Table{"a": &a};
//!
//! let x = b.a @;
//! let c = Table{"c": x};
//! b.a 3 !
//! what is c.c @ return?
//!
//! I think we need points and Tables to have pointers. then have methos to acces both the ref and the thing reffef
//!  the '.<ident>' returns a
//! [{a:&1}] | .a
//! [{a:&1}, &1] | @
//! [{a:&1}, 1]
//!
use std::{
    cell::RefCell,
    cmp::Ordering::{Equal, Greater, Less},
    collections::BTreeMap,
    sync::Arc,
};

#[derive(Debug, Clone, Eq)]
pub enum Value {
    Bool(bool),
    Int(i64),
    Ref(Arc<RefCell<Value>>),
    Str(String),
    Table(BTreeMap<Value, Arc<RefCell<Value>>>),
}
impl PartialOrd for Value {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Value {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match (self, other) {
            (Value::Bool(a), Value::Bool(b)) => a.cmp(b),
            (Value::Bool(_), Value::Int(_)) => std::cmp::Ordering::Greater,
            (Value::Bool(_), Value::Ref(_ref_cell)) => std::cmp::Ordering::Greater,
            (Value::Bool(_), Value::Str(_)) => std::cmp::Ordering::Greater,
            (Value::Bool(_), Value::Table(_btree_map)) => std::cmp::Ordering::Greater,
            (Value::Int(_), Value::Bool(_)) => std::cmp::Ordering::Less,
            (Value::Int(a), Value::Int(b)) => a.cmp(b),
            (Value::Int(_), Value::Ref(_ref_cell)) => std::cmp::Ordering::Greater,
            (Value::Int(_), Value::Str(_)) => std::cmp::Ordering::Greater,
            (Value::Int(_), Value::Table(_btree_map)) => std::cmp::Ordering::Greater,
            (Value::Ref(_ref_cell), Value::Bool(_)) => std::cmp::Ordering::Less,
            (Value::Ref(_ref_cell), Value::Int(_)) => std::cmp::Ordering::Less,
            (Value::Ref(a), Value::Ref(b)) => a.as_ref().cmp(b.as_ref()),
            (Value::Ref(_ref_cell), Value::Str(_)) => std::cmp::Ordering::Greater,
            (Value::Ref(_ref_cell), Value::Table(_btree_map)) => std::cmp::Ordering::Greater,
            (Value::Str(_), Value::Bool(_)) => Less,
            (Value::Str(_), Value::Int(_)) => Less,
            (Value::Str(_), Value::Ref(_ref_cell)) => Less,
            (Value::Str(a), Value::Str(b)) => a.cmp(b),
            (Value::Str(_), Value::Table(_btree_map)) => Greater,
            (Value::Table(_btree_map), Value::Bool(_)) => Less,
            (Value::Table(_btree_map), Value::Int(_)) => Less,
            (Value::Table(_btree_map), Value::Ref(_ref_cell)) => Less,
            (Value::Table(_btree_map), Value::Str(_)) => Less,
            (Value::Table(a), Value::Table(b)) => {
                if a == b {
                    return Equal;
                };
                let mut l = a.iter();
                let mut r = b.iter();
                loop {
                    match (l.next(), r.next()) {
                        (None, None) => return Equal,
                        (None, Some(_)) => return Less,
                        (Some(_), None) => return Greater,
                        (Some((x, xv)), Some((y, yv))) => match x.cmp(y) {
                            Less => return Less,
                            Equal => match xv.cmp(yv) {
                                Less => return Less,
                                Equal => (),
                                Greater => return Greater,
                            },
                            Greater => return Greater,
                        },
                    }
                }
            }
        }
    }
}
impl PartialEq for Value {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other) == Equal
        /*
        match (self, other) {
            (Self::Bool(l0), Self::Bool(r0)) => l0 == r0,
            (Self::Int(l0), Self::Int(r0)) => l0 == r0,
            (Self::Ref(l0), Self::Ref(r0)) => l0 == r0,
            (Self::Str(l0), Self::Str(r0)) => l0 == r0,
            (Self::Table(l0), Self::Table(r0)) => {
                for (l_key, l_value) in l0{
                    let Some(r_value) = r0.get(l_key) else {
                        return false
                    };
                    if ! Arc::ptr_eq(l_value, r_value) {
                        return false
                    }
                }
                true
            },
            _ => false,
        }
        */
    }
}

impl std::hash::Hash for Value {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        match self {
            Value::Bool(x) => x.hash(state),
            Value::Int(x) => x.hash(state),
            Value::Ref(ref_cell) => {
                let x = ref_cell.as_ref();
                x.borrow().hash(state)
            }
            Value::Str(x) => x.hash(state),
            Value::Table(btree_map) => {
                for key in btree_map.keys() {
                    key.hash(state);
                }
            }
        }
    }
}
