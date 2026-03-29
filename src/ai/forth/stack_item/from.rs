use std::sync::Arc;

use crate::ai::{forth::StackItem, BlackboardValue};
impl<EntityId> From<BlackboardValue<EntityId>> for StackItem<EntityId> {
    fn from(value: BlackboardValue<EntityId>) -> Self {
        match value {
            BlackboardValue::EntityId(entity) => Self::EntityId(entity),
            BlackboardValue::String(x) => Self::String(x),
            BlackboardValue::Coord { x, y } => Self::Coord { x, y: y },
        }
    }
}
impl<EntityId: Clone> From<&BlackboardValue<EntityId>> for StackItem<EntityId> {
    fn from(value: &BlackboardValue<EntityId>) -> Self {
        match value {
            BlackboardValue::EntityId(entity) => Self::EntityId(entity.clone()),
            BlackboardValue::String(x) => Self::String(x.clone()),
            BlackboardValue::Coord { x, y } => Self::Coord {
                x: x.clone(),
                y: y.clone(),
            },
        }
    }
}

impl<EntityId> From<bool> for StackItem<EntityId> {
    fn from(value: bool) -> Self {
        if value {
            Self::True
        } else {
            Self::False
        }
    }
}
impl<EntityId> From<&bool> for StackItem<EntityId> {
    fn from(value: &bool) -> Self {
        if *value {
            Self::True
        } else {
            Self::False
        }
    }
}

impl<EntityId> StackItem<EntityId> {
    pub fn from_entity(value: EntityId) -> Self {
        Self::EntityId(value)
    }
}
/*impl<EntityId> From<&EntityId> for StackItem<EntityId> {
    fn from(value: &EntityId) -> Self {
        Self::EntityId(*value)
    }
}*/

impl<EntityId> From<i32> for StackItem<EntityId> {
    fn from(value: i32) -> Self {
        Self::Int(value)
    }
}
impl<EntityId> From<&i32> for StackItem<EntityId> {
    fn from(value: &i32) -> Self {
        Self::Int(*value)
    }
}

impl<EntityId> From<usize> for StackItem<EntityId> {
    fn from(value: usize) -> Self {
        Self::Int(value as i32)
    }
}
impl<EntityId> From<&usize> for StackItem<EntityId> {
    fn from(value: &usize) -> Self {
        Self::Int(*value as i32)
    }
}

impl<EntityId> From<String> for StackItem<EntityId> {
    fn from(value: String) -> Self {
        Self::String(Arc::new(value))
    }
}
impl<EntityId> From<&str> for StackItem<EntityId> {
    fn from(value: &str) -> Self {
        Self::String(Arc::new(value.to_owned()))
    }
}

impl<EntityId, T: Into<StackItem<EntityId>>> From<Option<T>> for StackItem<EntityId> {
    fn from(value: Option<T>) -> Self {
        let Some(thing) = value else {
            return StackItem::False;
        };
        StackItem::some(thing.into())
    }
}
impl<'a, T, EntityId> From<&'a Option<T>> for StackItem<EntityId>
where
    &'a T: Into<StackItem<EntityId>>,
{
    fn from(value: &'a Option<T>) -> Self {
        value.as_ref().into()
    }
}
