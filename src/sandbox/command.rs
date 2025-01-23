use super::{EntityId, Item, Location};

mod use_object;

#[derive(Debug, PartialEq, PartialOrd, Clone)]
pub enum Command {
    AddItem { item: Item, loc: Location },
    Remove(EntityId),
    Damage { agent_id: EntityId, ammount: i16 },
    Rest { agent_id: EntityId, ammount: i16 },
    Heal { agent_id: EntityId, ammount: i16 },

    SetLocation { agent_id: EntityId, loc: Location },
}
