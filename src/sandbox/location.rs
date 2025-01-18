use super::EntityId;

#[derive(Debug, PartialEq, PartialOrd, Clone)]
pub enum Location {
    Inventory(EntityId),
    World { x: f32, y: f32 },
}
