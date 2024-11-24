#[derive(Debug, PartialEq, PartialOrd, Clone)]
pub enum Location {
    Inventory(usize),
    World{x:f32, y:f32},
}