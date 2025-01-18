use std::collections::HashMap;

use broad_phase::{Entity, EntityId as SpatialId, SpatialBloom};

use crate::sandbox::EntityId;

use super::moveit::Avalibility;

pub fn setup_avals_map(
    collisions: HashMap<EntityId, Entity>,
    rearendings: HashMap<EntityId, Entity>,
) -> (HashMap<SpatialId, Avalibility>, SpatialBloom) {
    let mut avals = HashMap::<SpatialId, Avalibility>::new();
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
