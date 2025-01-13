use std::collections::HashMap;

use crate::{sandbox::{Item, ItemClass, Location, ObjectId, Prev}, Vec2};

#[derive(Debug, PartialEq, Clone)]
pub struct World {
    pub(super) locations: HashMap<ObjectId, Location>,
    pub(super) energy: HashMap<ObjectId, i16>,
    pub(super) hp: HashMap<ObjectId, i16>,
    pub(super) sizes: HashMap<ObjectId, (f32, f32)>,
    pub(super) r#type: HashMap<ObjectId, Item>,
    pub(super) highest_id: ObjectId,
    pub(super) movement: HashMap<ObjectId, ((f32, f32), f32)>,
}

impl
    From<(
        HashMap<ObjectId, Location>,
        HashMap<ObjectId, i16>,
        HashMap<ObjectId, i16>,
        HashMap<ObjectId, (f32, f32)>,
        HashMap<ObjectId, Item>,
        HashMap<ObjectId, ((f32, f32), f32)>,
    )> for World
{
    fn from(
        (locations, energy, hp, sizes, r#type, movement): (
            HashMap<ObjectId, Location>,
            HashMap<ObjectId, i16>,
            HashMap<ObjectId, i16>,
            HashMap<ObjectId, (f32, f32)>,
            HashMap<ObjectId, Item>,
            HashMap<ObjectId, ((f32, f32), f32)>,
        ),
    ) -> Self {
        Self::new(locations, energy, hp, sizes, r#type, movement)
    }
}

impl World {
    pub fn find_nearest(&self, _entity_id: ObjectId, _item_class: &ItemClass) -> Option<Vec2> {
        todo!()
    }
    pub fn new_empty() -> Self {
        let locations = HashMap::new();
        let energy = HashMap::new();
        let hp = HashMap::new();
        let sizes = HashMap::new();
        let r#type = HashMap::new();
        let highest_id = 0;
        let movement = HashMap::new();
        World {
            locations,
            energy,
            hp,
            sizes,
            r#type,
            highest_id,
            movement,
        }
    }
    pub fn new(
        locations: HashMap<ObjectId, Location>,
        energy: HashMap<ObjectId, i16>,
        hp: HashMap<ObjectId, i16>,
        sizes: HashMap<ObjectId, (f32, f32)>,
        r#type: HashMap<ObjectId, Item>,
        movement: HashMap<ObjectId, ((f32, f32), f32)>,
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
        if let Some(value) = movement.keys().max() {
            highest_id = highest_id.max(value.clone());
        }

        Self {
            locations,
            energy,
            hp,
            sizes,
            r#type,
            highest_id,
            movement,
        }
    }
    pub fn get_energy(&self, id: &ObjectId) -> Option<&i16> {
        self.energy.get(id)
    }
    pub fn get_hp(&self, id: &ObjectId) -> Option<&i16> {
        self.hp.get(id)
    }
    pub fn get_location(&self, id: &ObjectId) -> Option<&Location> {
        self.locations.get(id)
    }
    pub fn insert_location(&mut self, id: ObjectId, loc: Location) -> Option<Location> {
        self.locations.insert(id, loc)
    }
    pub fn get_movement(&self, id: &ObjectId) -> Option<&((f32, f32), f32)> {
        self.movement.get(id)
    }
    pub fn insert_movement(
        &mut self,
        id: ObjectId,
        target: (f32, f32),
        speed: f32,
    ) -> Option<((f32, f32), f32)> {
        self.movement.insert(id, (target, speed))
    }
    pub fn get_size(&self, id: &ObjectId) -> Option<&(f32, f32)> {
        self.sizes.get(id)
    }
    pub fn get_type(&self, id: &ObjectId) -> Option<&Item> {
        self.r#type.get(id)
    }
    // interators
    pub fn energy_iter(&self) -> std::collections::hash_map::Iter<'_, usize, i16> {
        self.energy.iter()
    }
    pub fn hp_iter(&self) -> std::collections::hash_map::Iter<'_, usize, i16> {
        self.hp.iter()
    }
    pub fn locations_iter(&self) -> std::collections::hash_map::Iter<'_, usize, Location> {
        self.locations.iter()
    }
    pub fn movement_iter(&self) -> std::collections::hash_map::Iter<'_, usize, ((f32, f32), f32)> {
        self.movement.iter()
    }
    pub fn type_iter(&self) -> std::collections::hash_map::Iter<'_, ObjectId, Item> {
        self.r#type.iter()
    }
    //raws
    pub fn raw_sizes(&self) -> &HashMap<ObjectId, (f32, f32)> {
        &self.sizes
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

impl Prev for World {
    fn get_location(&self, id: &ObjectId) -> Option<(f32, f32)> {
        let Some(Location::World { x, y }) = self.get_location(id) else {
            return None;
        };
        Some((*x, *y))
    }

    fn get_size(&self, id: &ObjectId) -> Option<(f32, f32)> {
        let (x, y) = self.get_size(id)?;
        Some((*x, *y))
    }
}
