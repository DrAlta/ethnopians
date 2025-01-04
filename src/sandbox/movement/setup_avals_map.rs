use std::collections::HashMap;

use broad_phase::{Entity, EntityId, SpatialBloom};

use crate::sandbox::ObjectId;

use super::moveit::Avalibility;

pub fn setup_avals_map(
    collisions: HashMap<ObjectId, Entity>,
    rearendings: HashMap<ObjectId, Entity>,
) -> (HashMap<EntityId, Avalibility>, SpatialBloom) {
    let mut avals = HashMap::<EntityId, Avalibility>::new();
    let mut map = SpatialBloom::new(10.0, 10.0, Vec::new()).unwrap();
    for (unit_id, entity) in collisions {
        let entity_id = map.insert(entity);
        avals.insert(entity_id, Avalibility::Collision(unit_id));
    }

    for (unit_id, entity) in rearendings {
        let entity_id = map.insert(entity);
        avals.insert(entity_id, Avalibility::RearEnded(unit_id));
    }

    (avals, map)
}
