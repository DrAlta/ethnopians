use bevy::prelude::*;

use crate::sandbox::EntityId;

#[derive(Component, Debug, PartialEq, PartialOrd, Clone)]
pub enum Location {
    Inventory(EntityId),
    World { x: f32, y: f32 },
}
