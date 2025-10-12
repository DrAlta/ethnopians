use std::collections::HashMap;

//use broad_phase::{Entity, EntityId as SpatialId, SpatialBloom};

use crate::{
    broadphase::{Broadphase, SpatialId},
    types::AARect,
};

use crate::sandbox::EntityId;

use super::moveit::Avalibility;

pub fn setup_avals_map<B: Broadphase>(
    collisions: HashMap<EntityId, AARect>,
    rearendings: HashMap<EntityId, AARect>,
) -> (HashMap<SpatialId, Avalibility>, B) {
    let mut avals = HashMap::<SpatialId, Avalibility>::new();
    let mut map = B::new(Vec::new().into_iter());
    for (unit_id, entity) in collisions {
        let entity_id = map.insert(entity);
        avals.insert(entity_id, Avalibility::Collision(unit_id));
    }

    for (unit_id, entity) in rearendings {
        let entity_id = map.insert(entity);
        avals.insert(entity_id, Avalibility::RearEnded(unit_id));
    }
    map.ready();
    (avals, map)
}
