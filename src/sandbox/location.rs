use crate::{
    sandbox::{Component, EntityId},
    Number,
};

#[derive(Component, Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Location {
    Inventory(EntityId),
    World { x: Number, y: Number },
}
