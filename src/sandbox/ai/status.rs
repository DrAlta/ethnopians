use crate::sandbox::{ai::InpulseId, EntityId, Item};

#[derive(Debug, PartialEq)]
pub enum Status {
    Success,
    Failure,
    // FindNearest means the process is praying for the answer to 
    // "what is closest item of the specfified type to the location given" 
    /// to be put on the stack
    FindNearest{
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
    GetEntities{
        min_x: i32,
        min_y: i32,
        max_x: i32,
        max_y: i32,
    },
    // prey for the the items of the type to be removes from the array on top of the stack
    RemoveEntitiesOfType(Item),
    Running(InpulseId),
    None,
}
