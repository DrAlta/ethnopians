use std::{
    collections::BTreeMap,
    sync::{Arc, OnceLock},
};

use crate::sandbox::EntityId;

use super::table::TableInterior;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
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
    String(Arc<String>),
    Table(Arc<TableInterior>),
}
impl StackItem {
    pub fn success() -> StackItem {
        static SUCCESS: OnceLock<StackItem> = OnceLock::new();
        SUCCESS
            .get_or_init(|| StackItem::String(Arc::new("Success".to_owned())))
            .clone()
    }
    pub fn failure() -> StackItem {
        static FAILURE: OnceLock<StackItem> = OnceLock::new();
        FAILURE
            .get_or_init(|| StackItem::String(Arc::new("Failure".to_owned())))
            .clone()
    }
    pub fn init() -> StackItem {
        // static INIT: OnceLock<StackItem> = OnceLock::new();
        //  INIT.get_or_init(|| {
        StackItem::String(Arc::new("Init".to_owned()))
        // }).clone()
    }
    pub fn selector(value: i32) -> Self {
        static SELECTOR: OnceLock<StackItem> = OnceLock::new();

        let inner = TableInterior {
            map: BTreeMap::from([(
                SELECTOR.get_or_init(|| "Selector".into()).clone(),
                value.into(),
            )]),
        };
        Self::Table(Arc::new(inner))
    }
    pub fn sequence(value: i32) -> Self {
        static SEQUENCE: OnceLock<StackItem> = OnceLock::new();
        let inner = TableInterior {
            map: BTreeMap::from([(
                SEQUENCE.get_or_init(|| "Sequence".into()).clone(),
                value.into(),
            )]),
        };
        Self::Table(Arc::new(inner))
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
        StackItem::Table(Arc::new(TableInterior::new()))
    }
}

impl StackItem {
    pub fn stuff(
        &mut self,
        stuffing: StackItem,
        key: StackItem,
    ) -> Result<Option<StackItem>, String> {
        match self {
            StackItem::Table(stuffee) => {
                let x = Arc::make_mut(stuffee);
                Ok(x.insert(key, stuffing))
            }
            _ => Err("ForthKind::StuffeeNotTable".to_owned()),
        }
    }
}

impl<const N: usize> TryFrom<[(StackItem, StackItem); N]> for StackItem {
    type Error = String;

    fn try_from(value: [(StackItem, StackItem); N]) -> Result<StackItem, Self::Error> {
        let mut inner = TableInterior::new();
        for (key, stuffing) in value.into_iter() {
            inner.insert(key, stuffing);
        }
        Ok(Self::Table(Arc::new(inner)))
    }
}
impl<const N: usize> TryFrom<[(&str, StackItem); N]> for StackItem {
    type Error = String;

    fn try_from(value: [(&str, StackItem); N]) -> Result<StackItem, Self::Error> {
        let mut inner = TableInterior::new();
        for (key, stuffing) in value.into_iter() {
            inner.insert(key.into(), stuffing);
        }
        Ok(Self::Table(Arc::new(inner)))
    }
}

#[test]
fn fo() {
    assert_eq!(StackItem::success(), StackItem::success(),)
}
