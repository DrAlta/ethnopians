use bevy::prelude::*;

use crate::sandbox::EntityId;

#[derive(Event, Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct UseOnRequest {
    pub agent_id: EntityId,
    pub tool_id: EntityId,
    pub target_id: EntityId,
}
