use std::{
    collections::BTreeMap,
    sync::{Arc, OnceLock},
};

use super::table::TableInterior;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum StackItem<EntityId> {
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
    Option(Box<Self>),
    String(Arc<String>),
    Table(Arc<TableInterior<EntityId>>),
}
impl<EntityId> StackItem<EntityId> {
    pub fn success() -> Self {
        static SUCCESS: OnceLock<Arc<String>> = OnceLock::new();
        StackItem::String(
            SUCCESS
                .get_or_init(|| Arc::new("Success".to_owned()))
                .clone(),
        )
    }
    pub fn failure(_reason: String) -> Self {
        static FAILURE: OnceLock<Arc<String>> = OnceLock::new();
        StackItem::String(
            FAILURE
                .get_or_init(|| Arc::new("Failure".to_owned()))
                .clone(),
        )
    }
    pub fn init() -> Self {
        // static INIT: OnceLock<StackItem> = OnceLock::new();
        //  INIT.get_or_init(|| {
        StackItem::String(Arc::new("Init".to_owned()))
        // }).clone()
    }
}
impl<EntityId: Ord> StackItem<EntityId> {
    pub fn selector(value: i32) -> Self {
        static SELECTOR: OnceLock<Arc<String>> = OnceLock::new();

        let inner = TableInterior {
            map: BTreeMap::from([(
                StackItem::String(
                    SELECTOR
                        .get_or_init(|| Arc::new("Selector".to_owned()))
                        .clone(),
                ),
                value.into(),
            )]),
        };
        Self::Table(Arc::new(inner))
    }
    pub fn sequence(value: i32) -> Self {
        static SEQUENCE: OnceLock<Arc<String>> = OnceLock::new();
        let inner = TableInterior {
            map: BTreeMap::from([(
                StackItem::String(
                    SEQUENCE
                        .get_or_init(|| Arc::new("Sequence".to_owned()))
                        .clone(),
                ),
                value.into(),
            )]),
        };
        Self::Table(Arc::new(inner))
    }
}

impl<EntityId> StackItem<EntityId> {
    pub fn some(value: Self) -> Self {
        Self::Option(Box::new(value))
    }
    pub fn none() -> Self {
        Self::False
    }
    pub fn new_table() -> Self {
        StackItem::Table(Arc::new(TableInterior::new()))
    }
}

impl<EntityId: Ord> StackItem<EntityId> {
    pub fn from_iter<T: Into<Self>, I: Iterator<Item = T>>(value: I) -> Self {
        let mut inner = TableInterior::new();
        value.enumerate().for_each(|(idx, x)| {
            inner.insert(idx.into(), x.into());
        });
        Self::Table(Arc::new(inner))
    }

    pub fn stuff(&mut self, stuffing: Self, key: Self) -> Result<Option<Self>, String>
    where
        TableInterior<EntityId>: Clone,
    {
        match self {
            StackItem::Table(stuffee) => {
                let x = Arc::make_mut(stuffee);
                Ok(x.insert(key, stuffing))
            }
            _ => Err(format!(
                "{}:{}:ForthKind::StuffeeNotTable",
                file!(),
                line!()
            )),
        }
    }
}

impl<const N: usize, EntityId: Ord> TryFrom<[(StackItem<EntityId>, StackItem<EntityId>); N]>
    for StackItem<EntityId>
{
    type Error = String;

    fn try_from(
        value: [(StackItem<EntityId>, StackItem<EntityId>); N],
    ) -> Result<StackItem<EntityId>, Self::Error> {
        let mut inner = TableInterior::new();
        for (key, stuffing) in value.into_iter() {
            inner.insert(key, stuffing);
        }
        Ok(Self::Table(Arc::new(inner)))
    }
}
impl<const N: usize, EntityId: Ord> TryFrom<[(&str, StackItem<EntityId>); N]>
    for StackItem<EntityId>
{
    type Error = String;

    fn try_from(
        value: [(&str, StackItem<EntityId>); N],
    ) -> Result<StackItem<EntityId>, Self::Error> {
        let mut inner = TableInterior::new();
        for (key, stuffing) in value.into_iter() {
            inner.insert(key.into(), stuffing);
        }
        Ok(Self::Table(Arc::new(inner)))
    }
}

#[test]
fn fo() {
    assert_eq!(StackItem::<i8>::success(), StackItem::<i8>::success(),)
}
