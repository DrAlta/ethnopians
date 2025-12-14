use crate::sandbox::{EntityId, Message};

#[derive(Message, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Collision {
    pub agent_id: EntityId,
    pub collider_id: EntityId,
}
