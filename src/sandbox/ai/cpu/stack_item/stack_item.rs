use std::{cell::RefCell, collections::BTreeMap, rc::Rc};

use crate::sandbox::EntityId;

use super::table::{ParentTables, TableInterior};

impl StackItem {
    pub fn success() -> StackItem {
        StackItem::String("Success".to_owned())
    }
    pub fn failure() -> StackItem {
        StackItem::String("Failure".to_owned())
    }
    pub fn init() -> StackItem {
        StackItem::String("Init".to_owned())
    }
    pub fn selector(value: i32) -> Self {
        StackItem::try_from([("Selector", value.into())]).unwrap()
    }
    pub fn sequence(value: i32) -> Self {
        StackItem::try_from([("Sequence", value.into())]).unwrap()
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum StackItem {
    /*
    //Behaior states
    Sequence(usize),
    Selector(usize),
    // return statues
    Success,
    Failure,
    Init,
    */
    Int(i32),
    True,
    False,
    Coord { x: i32, y: i32 },
    EntityId(EntityId),
    //    Todo(Vec<EntityId>),
    // vvv sure to keep these vvvv
    Option(Box<StackItem>),
    String(String),
    Table(Rc<TableInterior>),
}
impl Clone for StackItem {
    fn clone(&self) -> Self {
        match self {
            StackItem::Table(rc) => StackItem::Table(Rc::clone(rc)),
            //StackItem::Sequence(x) => StackItem::Sequence(x.clone()),
            //StackItem::Selector(x) => StackItem::Selector(x.clone()),
            //StackItem::Success => StackItem::Success,
            //StackItem::Failure => StackItem::Failure,
            //StackItem::Init => StackItem::Init,
            StackItem::Int(x) => StackItem::Int(x.clone()),
            StackItem::True => StackItem::True,
            StackItem::False => StackItem::False,
            StackItem::Coord { x, y } => StackItem::Coord {
                x: x.clone(),
                y: y.clone(),
            },
            StackItem::String(x) => StackItem::String(x.clone()),
            StackItem::EntityId(entity) => StackItem::EntityId(entity.clone()),
            StackItem::Option(stack_item) => StackItem::Option(stack_item.clone()),
        }
    }
}
impl std::hash::Hash for StackItem {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        match self {
            StackItem::Table(rc) => {
                rc.hash(state);
            }
            /*
            StackItem::Sequence(x) => x.hash(state),
            StackItem::Selector(x) => x.hash(state),
            StackItem::Success => "Success".hash(state),
            StackItem::Failure => "Failure".hash(state),
            StackItem::Init => "Init".hash(state),
            */
            StackItem::Int(x) => x.hash(state),
            StackItem::True => true.hash(state),
            StackItem::False => false.hash(state),
            StackItem::Coord { x, y } => (x, y).hash(state),
            StackItem::String(x) => x.hash(state),
            StackItem::EntityId(x) => x.hash(state),
            StackItem::Option(x) => x.hash(state),
            //            StackItem::Todo(items) => todo!(),
        }
    }
}

impl StackItem {
    pub fn some(value: StackItem) -> StackItem {
        Self::Option(Box::new(value))
    }
    pub fn none() -> StackItem {
        Self::False
    }
    pub fn new_table() -> StackItem {
        StackItem::Table(Rc::new(TableInterior {
            map: RefCell::new(BTreeMap::new()),
            parents: RefCell::new(ParentTables::new()),
        }))
    }
}

impl StackItem {
    pub fn same(&self, other: &Self) -> bool {
        if let (StackItem::Table(self_rc), StackItem::Table(other_rc)) = (self, other) {
            return Rc::ptr_eq(&self_rc, &other_rc);
        }
        self.eq(other)
    }
    pub fn stuff(&mut self, stuffing: StackItem, key: StackItem) -> Result<(), String> {
        match (self, stuffing) {
            (StackItem::Table(stuffee), StackItem::Table(stuffing_rc)) => {
                if stuffee.has_ancester(&stuffing_rc) {
                    Err("ForthKind::CyclesNotAllowed".to_owned())
                } else {
                    stuffing_rc
                        .parents
                        .borrow_mut()
                        .push(Rc::downgrade(&stuffee));

                    stuffee
                        .map
                        .borrow_mut()
                        .insert(key, StackItem::Table(stuffing_rc));
                    Ok(())
                }
            }
            (StackItem::Table(stuffee), value) => {
                stuffee.map.borrow_mut().insert(key, value);
                Ok(())
            }
            _ => Err("ForthKind::StuffeeNotTable".to_owned()),
        }
    }
}

impl<const N: usize> TryFrom<[(StackItem, StackItem); N]> for StackItem {
    type Error = String;

    fn try_from(value: [(StackItem, StackItem); N]) -> Result<StackItem, Self::Error> {
        let mut ret = StackItem::new_table();
        for (key, stuffing) in value.into_iter() {
            ret.stuff(stuffing, key)?
        }
        Ok(ret)
    }
}
impl<const N: usize> TryFrom<[(&str, StackItem); N]> for StackItem {
    type Error = String;

    fn try_from(value: [(&str, StackItem); N]) -> Result<StackItem, Self::Error> {
        let mut ret = StackItem::new_table();
        for (key, stuffing) in value.into_iter() {
            ret.stuff(stuffing, Self::String(key.to_owned()))?
        }
        Ok(ret)
    }
}

impl From<i32> for StackItem {
    fn from(value: i32) -> Self {
        Self::Int(value)
    }
}
impl From<String> for StackItem {
    fn from(value: String) -> Self {
        Self::String(value)
    }
}
impl From<&str> for StackItem {
    fn from(value: &str) -> Self {
        Self::String(value.to_owned())
    }
}
