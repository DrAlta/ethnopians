use std::collections::BTreeSet;

use crate::{
    broadphase::{
        sweep_and_prune::{sortie, Entry},
        Broadphase, SpatialId,
    },
    types::AARect,
    Number,
};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SweepAndPrune {
    pub sorted: Vec<Entry>,
    pub dirty: bool,
}

impl SweepAndPrune {
    pub fn new<I: Iterator<Item = AARect>>(entities: I) -> Self {
        let sorted: Vec<Entry> = entities
            .enumerate()
            .map(|(idx, aabb)| Entry {
                aabb,
                entity_id: idx,
            })
            .collect();
        Self {
            sorted,
            dirty: true,
        }
    }
    pub fn insert(&mut self, aabb: AARect) -> SpatialId {
        let entity_id = self.sorted.len();
        self.sorted.push(Entry { aabb, entity_id });
        self.dirty = true;
        entity_id
    }
    pub fn get_entity(&self, k: &SpatialId) -> Option<AARect> {
        let found = self
            .sorted
            .iter()
            .find(|Entry { entity_id, .. }| entity_id == k)?;
        Some(found.aabb.clone())
    }
    pub fn qurry(
        &self,
        min_x: Number,
        min_y: Number,
        max_x: Number,
        max_y: Number,
    ) -> BTreeSet<SpatialId> {
        let mut ret = BTreeSet::new();
        let mut temp;
        let sorted = if self.dirty {
            temp = self.sorted.clone();
            temp.sort_by(sortie);
            &temp
        } else {
            &self.sorted
        };
        for i in 0..sorted.len() {
            let Entry { aabb, entity_id } = &sorted[i];

            if aabb.min_x > max_x {
                break;
            }

            if max_y > aabb.min_y
                && aabb.min_y + aabb.height > min_y
                && max_x > aabb.min_x
                && aabb.min_x + aabb.width > min_x
            {
                ret.insert(*entity_id);
            }
        }
        ret
    }
    pub fn ready(&mut self) -> bool {
        if self.dirty {
            self.sorted.sort_by(sortie);
            true
        } else {
            false
        }
    }
    pub fn collisions(&mut self) -> BTreeSet<SpatialId> {
        let mut ret = BTreeSet::new();
        if self.dirty {
            self.sorted.sort_by(sortie);
        };
        for i in 0..self.sorted.len() {
            let Entry {
                aabb: one,
                entity_id: one_id,
            } = &self.sorted[i];
            //let one_left = one.min_x;
            let one_right = one.min_x + one.width;

            for j in i..self.sorted.len() {
                let Entry {
                    aabb: two,
                    entity_id: two_id,
                } = &self.sorted[j];
                let two_left = two.min_x;
                //let two_right = two.min_x + two.width;

                if two_left > one_right {
                    break;
                }

                if one.min_y + one.height > two.min_y && two.min_y + two.height > one.min_y {
                    ret.insert(*one_id);
                    ret.insert(*two_id);
                }
            }
        }
        ret
    }
}

impl Broadphase for SweepAndPrune {
    fn new<I: Iterator<Item = AARect>>(entities: I) -> Self {
        SweepAndPrune::new(entities)
    }

    fn insert(&mut self, aabb: AARect) -> SpatialId {
        SweepAndPrune::insert(self, aabb)
    }

    fn ready(&mut self) -> bool {
        SweepAndPrune::ready(self)
    }

    fn qurry(
        &self,
        min_x: Number,
        min_y: Number,
        max_x: Number,
        max_y: Number,
    ) -> BTreeSet<SpatialId> {
        SweepAndPrune::qurry(self, min_x, min_y, max_x, max_y)
    }

    fn get_entity(&self, k: &SpatialId) -> Option<AARect> {
        SweepAndPrune::get_entity(self, k)
    }
}
