use std::collections::HashMap;

use super::{Item, Location, ObjectId};

mod execute_commands;

#[derive(Debug, PartialEq, Clone)]
pub struct World {
    locations: HashMap<ObjectId, Location>,
    energy: HashMap<ObjectId, i16>,
    hp: HashMap<ObjectId, i16>,
    sizes: HashMap<ObjectId, (f32, f32)>,
    r#type: HashMap<ObjectId, Item>,
    highest_id: ObjectId,
}

impl
    From<(
        HashMap<ObjectId, Location>,
        HashMap<ObjectId, i16>,
        HashMap<ObjectId, i16>,
        HashMap<ObjectId, (f32, f32)>,
        HashMap<ObjectId, Item>,
    )> for World
{
    fn from(
        (locations, energy, hp, sizes, r#type): (
            HashMap<ObjectId, Location>,
            HashMap<ObjectId, i16>,
            HashMap<ObjectId, i16>,
            HashMap<ObjectId, (f32, f32)>,
            HashMap<ObjectId, Item>,
        ),
    ) -> Self {
        let mut highest_id = 0;
        if let Some(value) = locations.keys().max() {
            highest_id = value.clone();
        }
        if let Some(value) = energy.keys().max() {
            highest_id = highest_id.max(value.clone());
        }
        if let Some(value) = hp.keys().max() {
            highest_id = highest_id.max(value.clone());
        }
        if let Some(value) = sizes.keys().max() {
            highest_id = highest_id.max(value.clone());
        }
        if let Some(value) = r#type.keys().max() {
            highest_id = highest_id.max(value.clone());
        }

        Self {
            locations,
            energy,
            hp,
            sizes,
            r#type,
            highest_id,
        }
    }
}

impl World {
    pub fn get_location(&self, id: &ObjectId) -> Option<&Location> {
        self.locations.get(id)
    }
    pub fn get_energy(&self, id: &ObjectId) -> Option<&i16> {
        self.energy.get(id)
    }
    pub fn get_hp(&self, id: &ObjectId) -> Option<&i16> {
        self.hp.get(id)
    }
    pub fn get_size(&self, id: &ObjectId) -> Option<&(f32, f32)> {
        self.sizes.get(id)
    }
    pub fn get_type(&self, id: &ObjectId) -> Option<&Item> {
        self.r#type.get(id)
    }
    pub fn type_iter(&self) -> std::collections::hash_map::Iter<'_, ObjectId, Item> {
        self.r#type.iter()
    }
}

impl World {
    pub fn get_new_object_id(&self) -> ObjectId {
        let mut new_id = self.highest_id.clone();
        loop {
            new_id += 1;
            if self.locations.contains_key(&new_id) {
                continue;
            }
            if self.energy.contains_key(&new_id) {
                continue;
            }
            if self.hp.contains_key(&new_id) {
                continue;
            }
            if self.sizes.contains_key(&new_id) {
                continue;
            }
            if self.r#type.contains_key(&new_id) {
                continue;
            }
            return new_id;
        }
    }
}
