use crate::sandbox::{EntityId, Item, InpulseId};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Prayer{
    Running(InpulseId),
    FindInInventory {
        item_class: Item,
    },
    // UseOn(ToolId, TargetId)
    UseOn(EntityId, EntityId),
    // FindNearest means the process is praying for the answer to
    // "what is closest item of the specfified type to the location given"
    /// to be put on the stack
    FindNearest {
        x: i32,
        y: i32,
        item_class: Item,
    },
    // GetEnergy means the process is praying for the answer to
    // "what is the energy of the specfified entity"
    /// to be put on the stack
    GetEnergy(EntityId),
    // GetLocation means the process is praying for the answer to
    // "what is the location of the specfified entity"
    /// to be put on the stack
    GetLocation(EntityId),
    // GetHp means the process is praying for the answer to
    // "what is the Hp of the specfified entity"
    /// to be put on the stack
    GetHp(EntityId),
    // takes a Blackboard key that points to an ItemClass and u8 of the number to compare to
    GetIsInventoryGE {
        agent: EntityId,
        item_class: Item,
        amount: i32,
    },
    GetEntities {
        min_x: i32,
        min_y: i32,
        max_x: i32,
        max_y: i32,
    },
    // prey for the the items of the type to be removes from the array on top of the stack
    RemoveEntitiesOfType(Item),
    RetainEntitiesOfType(Item),
}