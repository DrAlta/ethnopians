use bevy::prelude::*;

use crate::sandbox::{world::Movement, EntityId};

#[derive(Event, Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct GotoRequest {
    pub action_id: u64,
    pub agent_id: EntityId,
    pub movement: Movement,
}
