use crate::sandbox::{ai::StackItem, EntityId};
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Hash)]
pub enum BlackboardValue {
    EntityId(EntityId),
    String(String),
    Coord { x: i32, y: i32 },
}
impl From<StackItem> for BlackboardValue {
    fn from(value: StackItem) -> Self {
        match value {
            StackItem::EntityId(entity) => BlackboardValue::EntityId(entity),
            StackItem::String(x) => BlackboardValue::String(x),
            StackItem::True => BlackboardValue::String("True".to_owned()),
            StackItem::False => BlackboardValue::String("False".to_owned()),
            StackItem::Coord { x, y } => BlackboardValue::Coord { x, y },
            x @ StackItem::Int(_) | x @ StackItem::Option(_) | x @ StackItem::Table(_) => {
                BlackboardValue::String(format!("{x}"))
            }
        }
    }
}
