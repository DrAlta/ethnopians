use bevy::prelude::*;

use crate::sandbox::EntityId;

#[derive(Debug,PartialEq,Event)]
pub struct ChangeEnergy{
    pub entity_id: EntityId,
    pub delta: i32
}