use crate::sandbox::ObjectId;
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Hash)]
pub enum BlackboardValue {
    EntityId(ObjectId),
}
