use std::collections::BTreeSet;

use bevy::prelude::*;

use crate::sandbox::EntityId;

#[derive(Component, Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Inventory(pub BTreeSet<EntityId>);
