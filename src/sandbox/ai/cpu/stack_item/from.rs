use std::sync::Arc;

use crate::sandbox::ai::{BlackboardValue, StackItem};

impl From<i32> for StackItem {
    fn from(value: i32) -> Self {
        Self::Int(value)
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
impl From<BlackboardValue> for StackItem {
    fn from(value: BlackboardValue) -> Self {
        match value {
            BlackboardValue::EntityId(entity) => Self::EntityId(entity),
            BlackboardValue::String(x) => Self::String(x),
            BlackboardValue::Coord { x, y } => Self::Coord { x, y: y },
        }
    }
}
