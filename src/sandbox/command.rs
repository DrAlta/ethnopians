use bevy::prelude::*;
mod foo;


use crate::sandbox::{EntityId, Item, Location};

mod use_object;
pub use use_object::use_object_system;

#[derive(Debug, PartialEq, PartialOrd, Clone)]
pub enum Command {
    AddItem { item: Item, loc: Location },
    Remove(EntityId),
    Damage { agent_id: EntityId, ammount: i32 },
    Rest { agent_id: EntityId, ammount: i32 },
    Heal { agent_id: EntityId, ammount: i32 },

    SetLocation { agent_id: EntityId, loc: Location },
}

#[derive(Debug, PartialEq, PartialOrd, Clone)]
pub enum ActionId {
    UseObject,
}


#[derive(Event, Debug)]
pub struct PosibleActionsRequest {
    pub agent_id: EntityId,
    pub target_id: EntityId,
}
#[derive(Event, Debug)]
pub struct PosibleActionsResponce {
    pub agent_id: EntityId,
    pub target_id: EntityId,
    pub action_id: ActionId,
}