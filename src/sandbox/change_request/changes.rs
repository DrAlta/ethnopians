use bevy::prelude::*;
use macros::Structs;

use crate::sandbox::{EntityId, Item, Location, change_request::Dispatch};

//use super::dispatch::Dispatch2;

#[derive(Debug, Clone, PartialEq, Structs)]
pub enum Changes {
    Despawn(EntityId),
    Energy { entity_id: EntityId, delta: i32 },
    Hp { entity_id: EntityId, delta: i32 },
    SpawnLocationType { location: Location, tyep: Item },
}

