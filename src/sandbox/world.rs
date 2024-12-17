use std::collections::HashMap;

use super::{Item, Location, ObjectId};

#[derive(Debug, PartialEq, Clone)]
pub struct World {
    pub locations: HashMap<ObjectId, Location>,
    pub energy: HashMap<ObjectId, i16>,
    pub hp: HashMap<ObjectId, i16>,
    pub sizes: HashMap<ObjectId, (f32, f32)>,
    pub r#type: HashMap<ObjectId, Item>,
}
