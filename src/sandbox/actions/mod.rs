use crate::sandbox::{EntityId, Item, Location};

mod action_id;
pub use action_id::ActionId;
mod posible_actions_request;
pub use posible_actions_request::PosibleActionsRequest;
mod posible_actions_responce;
pub use posible_actions_responce::PosibleActionsResponce;
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
