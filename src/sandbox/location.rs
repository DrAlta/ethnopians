use super::ObjectId;

#[derive(Debug, PartialEq, PartialOrd, Clone)]
pub enum Location {
    Inventory(ObjectId),
    World { x: f32, y: f32 },
}
