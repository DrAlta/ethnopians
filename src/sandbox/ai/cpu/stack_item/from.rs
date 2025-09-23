use std::sync::Arc;

use crate::sandbox::{
    ai::{BlackboardValue, StackItem},
    EntityId,
};

impl From<BlackboardValue> for StackItem {
    fn from(value: BlackboardValue) -> Self {
        match value {
            BlackboardValue::EntityId(entity) => Self::EntityId(entity),
            BlackboardValue::String(x) => Self::String(x),
            BlackboardValue::Coord { x, y } => Self::Coord { x, y: y },
        }
    }
}
impl From<&BlackboardValue> for StackItem {
    fn from(value: &BlackboardValue) -> Self {
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

impl From<bool> for StackItem {
    fn from(value: bool) -> Self {
        if value {
            Self::True
        } else {
            Self::False
        }
    }
}
impl From<&bool> for StackItem {
    fn from(value: &bool) -> Self {
        (*value).into()
    }
}

impl From<EntityId> for StackItem {
    fn from(value: EntityId) -> Self {
        Self::EntityId(value)
    }
}
impl From<&EntityId> for StackItem {
    fn from(value: &EntityId) -> Self {
        Self::EntityId(*value)
    }
}

impl From<i32> for StackItem {
    fn from(value: i32) -> Self {
        Self::Int(value)
    }
}
impl From<&i32> for StackItem {
    fn from(value: &i32) -> Self {
        Self::Int(*value)
    }
}

impl From<usize> for StackItem {
    fn from(value: usize) -> Self {
        Self::Int(value as i32)
    }
}
impl From<&usize> for StackItem {
    fn from(value: &usize) -> Self {
        Self::Int(*value as i32)
    }
}

impl From<String> for StackItem {
    fn from(value: String) -> Self {
        Self::String(Arc::new(value))
    }
}
impl From<&str> for StackItem {
    fn from(value: &str) -> Self {
        Self::String(Arc::new(value.to_owned()))
    }
}

impl<T: Into<StackItem>> From<Option<T>> for StackItem {
    fn from(value: Option<T>) -> Self {
        let Some(thing) = value else {
            return StackItem::False;
        };
        StackItem::some(thing.into())
    }
}
impl<'a, T> From<&'a Option<T>> for StackItem
where
    &'a T: Into<StackItem>,
{
    fn from(value: &'a Option<T>) -> Self {
        value.as_ref().into()
    }
}
