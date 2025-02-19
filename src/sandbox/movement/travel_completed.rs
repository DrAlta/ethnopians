use bevy::prelude::*;

use crate::sandbox::EntityId;

#[derive(Debug, Event, PartialEq)]
pub struct TravelCompleted {
    pub entity_id: EntityId,
}
