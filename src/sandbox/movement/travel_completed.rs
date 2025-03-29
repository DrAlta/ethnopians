use bevy::prelude::*;

use crate::sandbox::EntityId;

#[derive(Event, Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct TravelCompleted {
    pub entity_id: EntityId,
}
