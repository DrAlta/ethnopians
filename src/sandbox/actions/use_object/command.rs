use crate::sandbox::{
    EntityId, Item,
};

#[derive(Debug, PartialEq, PartialOrd, Clone)]
pub enum Command {
    RemoveAndAddToInvetory{
        remove:EntityId,
        inventory: EntityId,
        item: Item,
    },
    Heal{
        agent_id: EntityId,
        energy: i32,
        hp: i32,
    },
    Rest {
        agent_id: EntityId,
        amount: i32
    }
}

