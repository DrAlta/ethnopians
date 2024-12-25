use std::collections::HashMap;

use crate::sandbox::{Location, ObjectId, Prev};

pub struct WorldWraper<'a> {
    locations: &'a HashMap<ObjectId, Location>,
    sizes: &'a HashMap<ObjectId, (f32, f32)>,
}

impl<'a> Prev for WorldWraper<'a> {
    fn get_location(&self, id: &ObjectId) -> Option<(f32, f32)> {
        let Some(Location::World { x, y }) = self.locations.get(id) else {
            return None;
        };
        Some((*x, *y))

    }

    fn get_size(&self, id: &ObjectId) -> Option<(f32, f32)> {
        let (x, y) = self.sizes.get(id)?;
        Some((*x, *y))
    }
} 