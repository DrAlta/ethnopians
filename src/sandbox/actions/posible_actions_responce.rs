use bevy::prelude::*;

use crate::sandbox::{ActionId, EntityId};

#[derive(Event, Debug, PartialEq, Eq)]
pub struct PosibleActionsResponce {
    pub agent_id: EntityId,
    pub target_id: EntityId,
    pub action_id: ActionId,
}
