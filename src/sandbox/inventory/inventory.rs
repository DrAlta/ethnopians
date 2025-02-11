use std::collections::BTreeSet;

use bevy::prelude::*;

use crate::sandbox::EntityId;

#[derive(Component)]
pub struct Inventory(pub BTreeSet<EntityId>);
