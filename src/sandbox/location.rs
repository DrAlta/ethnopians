use bevy::prelude::*;

use crate::{sandbox::EntityId, Number};

#[derive(Component, Debug, PartialEq, PartialOrd, Clone)]
pub enum Location {
    Inventory(EntityId),
    World { x: Number, y: Number },
}
