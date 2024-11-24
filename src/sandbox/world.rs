use std::collections::HashMap;

use super::{Item, Location};


#[derive(Debug, PartialEq, Clone)]
pub struct World{
    pub locations: HashMap<usize, Location>,
    pub energy: HashMap<usize, i16>,
    pub hp: HashMap<usize, i16>,
    pub r#type: HashMap<usize, Item>,
}
