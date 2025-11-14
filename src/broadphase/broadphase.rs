use std::collections::BTreeSet;

use crate::{types::AARect, Number};

pub trait Broadphase<SpatialId> {
    fn new<I: Iterator<Item = AARect>>(entities: I) -> Self;
    fn insert(&mut self, aabb: AARect) -> SpatialId;
    fn ready(&mut self) -> bool;
    fn qurry(
        &self,
        min_x: Number,
        min_y: Number,
        max_x: Number,
        max_y: Number,
    ) -> BTreeSet<SpatialId>;
    fn get_entity(&self, k: &SpatialId) -> Option<AARect>;
}
