use std::collections::HashMap;

use broad_phase::{AARect, Entity, SpatialBloom};

use crate::{
    sandbox::{EntityId, Item, ItemClass, Location, Prev},
    Vec2,
};

#[derive(Debug, PartialEq, Clone)]
pub struct World {
    pub(super) locations: HashMap<EntityId, Location>,
    pub(super) energy: HashMap<EntityId, i16>,
    pub(super) hp: HashMap<EntityId, i16>,
    pub(super) sizes: HashMap<EntityId, (f32, f32)>,
    pub(super) r#type: HashMap<EntityId, Item>,
    pub(super) highest_id: EntityId,
    pub(super) movement: HashMap<EntityId, ((f32, f32), f32)>,
    pub(super) spatial_bloom_maybe: Option<(SpatialBloom, HashMap<broad_phase::EntityId, EntityId>)>,
}

impl
    From<(
        HashMap<EntityId, Location>,
        HashMap<EntityId, i16>,
        HashMap<EntityId, i16>,
        HashMap<EntityId, (f32, f32)>,
        HashMap<EntityId, Item>,
        HashMap<EntityId, ((f32, f32), f32)>,
    )> for World
{
    fn from(
        (locations, energy, hp, sizes, r#type, movement): (
            HashMap<EntityId, Location>,
            HashMap<EntityId, i16>,
            HashMap<EntityId, i16>,
            HashMap<EntityId, (f32, f32)>,
            HashMap<EntityId, Item>,
            HashMap<EntityId, ((f32, f32), f32)>,
        ),
    ) -> Self {
        Self::new(locations, energy, hp, sizes, r#type, movement)
    }
}

impl World {
    pub fn get_spatial_bloom(&self) -> Option<&(SpatialBloom, HashMap<broad_phase::EntityId, EntityId>)> {
        self.spatial_bloom_maybe.as_ref()
    }
    pub fn pop_spatial_bloom(&mut self) -> &(SpatialBloom, HashMap<broad_phase::EntityId, EntityId>) {
        let mut map = HashMap::new();
        let mut sb = SpatialBloom::new(10.0, 10.0, Vec::new()).unwrap();
 
        for (k,v) in self.locations.iter() {
            let Location::World { x, y } = v else {
                continue;
            };
            let Some((width, height)) = self.sizes.get(k) else {
                continue;
            };
            let sb_entity = Entity::AARect(AARect::new(*x, *y, *width, *height));
            let sb_id = sb.insert(sb_entity);
            map.insert(sb_id, k.clone());
        };
        self.spatial_bloom_maybe = Some((sb, map));
        self.spatial_bloom_maybe.as_ref().unwrap()
    }
    pub fn find_nearest(&self, _pos: Vec2, _item_class: &ItemClass) -> Option<EntityId> {
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
            spatial_bloom_maybe: None,
        }
    }
    pub fn new(
        locations: HashMap<EntityId, Location>,
        energy: HashMap<EntityId, i16>,
        hp: HashMap<EntityId, i16>,
        sizes: HashMap<EntityId, (f32, f32)>,
        r#type: HashMap<EntityId, Item>,
        movement: HashMap<EntityId, ((f32, f32), f32)>,
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
            spatial_bloom_maybe: None
        }
    }
    pub fn get_energy(&self, id: &EntityId) -> Option<&i16> {
        self.energy.get(id)
    }
    pub fn get_hp(&self, id: &EntityId) -> Option<&i16> {
        self.hp.get(id)
    }
    pub fn get_location(&self, id: &EntityId) -> Option<&Location> {
        self.locations.get(id)
    }
    pub fn insert_location(&mut self, id: EntityId, loc: Location) -> Option<Location> {
        self.locations.insert(id, loc)
    }
    pub fn get_movement(&self, id: &EntityId) -> Option<&((f32, f32), f32)> {
        self.movement.get(id)
    }
    pub fn insert_movement(
        &mut self,
        id: EntityId,
        target: (f32, f32),
        speed: f32,
    ) -> Option<((f32, f32), f32)> {
        self.movement.insert(id, (target, speed))
    }
    pub fn get_size(&self, id: &EntityId) -> Option<&(f32, f32)> {
        self.sizes.get(id)
    }
    pub fn get_type(&self, id: &EntityId) -> Option<&Item> {
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
    pub fn type_iter(&self) -> std::collections::hash_map::Iter<'_, EntityId, Item> {
        self.r#type.iter()
    }
    //raws
    pub fn raw_sizes(&self) -> &HashMap<EntityId, (f32, f32)> {
        &self.sizes
    }
}

impl World {
    pub fn get_new_entity_id(&self) -> EntityId {
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
    fn get_location(&self, id: &EntityId) -> Option<(f32, f32)> {
        let Some(Location::World { x, y }) = self.get_location(id) else {
            return None;
        };
        Some((*x, *y))
    }

    fn get_size(&self, id: &EntityId) -> Option<(f32, f32)> {
        let (x, y) = self.get_size(id)?;
        Some((*x, *y))
    }
}
