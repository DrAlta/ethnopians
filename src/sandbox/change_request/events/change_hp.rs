use bevy::prelude::*;

use crate::sandbox::EntityId;

#[derive(Debug,PartialEq,Event)]
pub struct ChangeHp{
    pub entity_id: EntityId,
    pub delta: i32
}