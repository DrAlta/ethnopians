use bevy::prelude::*;

use crate::sandbox::EntityId;

#[derive(Debug, Event, PartialEq)]
pub struct Collision {
    pub agent_id: EntityId,
    pub collider_id: EntityId,
}
