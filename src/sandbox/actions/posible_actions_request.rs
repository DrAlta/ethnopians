use bevy::prelude::*;

use crate::sandbox::EntityId;

#[derive(Event, Debug)]
pub struct PosibleActionsRequest {
    pub agent_id: EntityId,
    pub target_id: EntityId,
}
