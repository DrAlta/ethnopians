use super::{Item, Location, ObjectId};

mod use_object;

#[derive(Debug, PartialEq, PartialOrd, Clone)]
pub enum Command {
    AddItem { item: Item, loc: Location },
    Remove(ObjectId),
    Damage { agent_id: ObjectId, ammount: i16 },
    Rest { agent_id: ObjectId, ammount: i16 },
    Heal { agent_id: ObjectId, ammount: i16 },

    SetLocation { agent_id: ObjectId, loc: Location },
}
