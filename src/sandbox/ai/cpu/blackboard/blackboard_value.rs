use std::sync::{Arc, OnceLock};

use crate::sandbox::{ai::StackItem, EntityId};
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Hash)]
pub enum BlackboardValue {
    EntityId(EntityId),
    String(Arc<String>),
    Coord { x: i32, y: i32 },
}

impl From<&str> for BlackboardValue {
    fn from(value: &str) -> Self {
        BlackboardValue::String(Arc::new(value.to_owned()))
    }
}

impl From<StackItem> for BlackboardValue {
    fn from(value: StackItem) -> Self {
        match value {
            StackItem::EntityId(entity) => BlackboardValue::EntityId(entity),
            StackItem::String(x) => BlackboardValue::String(x),
            StackItem::True => {
                static TRUE: OnceLock<BlackboardValue> = OnceLock::new();
                TRUE.get_or_init(|| BlackboardValue::String(Arc::new("True".to_owned())))
                    .clone()
            }
            StackItem::False => {
                static FALSE: OnceLock<BlackboardValue> = OnceLock::new();
                FALSE
                    .get_or_init(|| BlackboardValue::String(Arc::new("False".to_owned())))
                    .clone()
            }
            StackItem::Coord { x, y } => BlackboardValue::Coord { x, y },
            x @ StackItem::Int(_) | x @ StackItem::Option(_) | x @ StackItem::Table(_) => {
                BlackboardValue::String(Arc::new(format!("{x}")))
            }
        }
    }
}
impl TryInto<EntityId> for BlackboardValue {
    type Error = ();

    fn try_into(self) -> Result<EntityId, Self::Error> {
        match self{
            BlackboardValue::EntityId(entity) => Ok(entity),
            BlackboardValue::String(_) |
            BlackboardValue::Coord { .. } => Err(()),
        }
    }
}
impl TryInto<EntityId> for &BlackboardValue {
    type Error = ();

    fn try_into(self) -> Result<EntityId, Self::Error> {
        match self{
            BlackboardValue::EntityId(entity) => Ok(*entity),
            BlackboardValue::String(_) |
            BlackboardValue::Coord { .. } => Err(()),
        }
    }
}
