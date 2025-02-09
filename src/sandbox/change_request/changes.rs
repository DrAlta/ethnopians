use bevy::prelude::*;
use macros::Structs;

use crate::sandbox::{EntityId, Item, Location};

#[derive(Debug,PartialEq, Structs)]
pub enum Changes{
    Despawn(EntityId),
    Energy{entity_id: EntityId, delta: i32},
    Hp{entity_id: EntityId, delta: i32},
    SpawnLocationType{location: Location, tyep: Item},

}

pub trait Dispatch {
    fn dispatch(self, commands: &mut Commands);
}

impl<T:Dispatch> Dispatch for Vec<T> {
    fn dispatch(self, commands: &mut Commands) {
        self.into_iter().for_each(|x| x.dispatch(commands));
    }
}