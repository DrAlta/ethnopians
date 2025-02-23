use crate::sandbox::EntityId;
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Hash)]
pub enum BlackboardValue {
    EntityId(EntityId),
    String(String),
}
