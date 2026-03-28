use std::sync::{Arc, OnceLock};

use crate::ai::forth::StackItem;
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Hash)]
pub enum BlackboardValue<EntityId> {
    EntityId(EntityId),
    String(Arc<String>),
    Coord { x: i32, y: i32 },
}

impl<EntityId> From<&str> for BlackboardValue<EntityId> {
    fn from(value: &str) -> Self {
        BlackboardValue::String(Arc::new(value.to_owned()))
    }
}
impl<EntityId> /* From<EntityId> for*/ BlackboardValue<EntityId> {
    pub fn from_entity(value: EntityId) -> Self {
        BlackboardValue::EntityId(value)
    }
}

impl<EntityId: std::fmt::Display> From<StackItem<EntityId>> for BlackboardValue<EntityId> {
    fn from(value: StackItem<EntityId>) -> Self {
        match value {
            StackItem::EntityId(entity) => BlackboardValue::EntityId(entity),
            StackItem::String(x) => BlackboardValue::String(x),
            StackItem::True => {
                static TRUE: OnceLock<Arc::<String>> = OnceLock::new();
                BlackboardValue::String(
                    TRUE.get_or_init(
                        || 
                        Arc::new("True".to_owned())
                    )
                    .clone()
               )
            }
            StackItem::False => {
                static FALSE: OnceLock<Arc::<String>> = OnceLock::new();
                BlackboardValue::String(
                    FALSE.get_or_init(
                        || 
                        Arc::new("False".to_owned())
                    )
                    .clone()
                )
            }
            StackItem::Coord { x, y } => BlackboardValue::Coord { x, y },
            x @ StackItem::Int(_) | x @ StackItem::Option(_) | x @ StackItem::Table(_) => {
                BlackboardValue::String(Arc::new(format!("{x}")))
            }
        }
    }
}
impl<EntityId> /*TryInto<EntityId> for*/ BlackboardValue<EntityId> {
    ///type Error = ();
    pub fn try_into_entity(self) -> Result<EntityId, ()> {
        match self {
            BlackboardValue::EntityId(entity) => Ok(entity),
            BlackboardValue::String(_) | BlackboardValue::Coord { .. } => Err(()),
        }
    }
}
/*
impl<EntityId: Clone> TryInto<EntityId> for &BlackboardValue<EntityId> {
    type Error = ();

    fn try_into_entity(self) -> Result<EntityId, ()> {
        match self {
            BlackboardValue::EntityId(entity) => Ok(entity.clone()),
            BlackboardValue::String(_) | BlackboardValue::Coord { .. } => Err(()),
        }
    }
}
    */
