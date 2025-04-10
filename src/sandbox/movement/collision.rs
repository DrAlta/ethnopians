use crate::sandbox::{EntityId, Event};

#[derive(Event, Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Collision {
    pub agent_id: EntityId,
    pub collider_id: EntityId,
}
