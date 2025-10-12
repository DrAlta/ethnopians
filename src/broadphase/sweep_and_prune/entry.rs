use crate::{broadphase::SpatialId, types::AARect};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Entry {
    pub aabb: AARect,
    pub entity_id: SpatialId,
}
