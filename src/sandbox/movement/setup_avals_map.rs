use std::collections::HashMap;

//use broad_phase::{Entity, EntityId as SpatialId, SpatialBloom};

use crate::{sweep_and_prune::{SpatialId, SweepAndPrune}, types::AARect};

use crate::sandbox::EntityId;

use super::moveit::Avalibility;

pub fn setup_avals_map(
    collisions: HashMap<EntityId, AARect>,
    rearendings: HashMap<EntityId, AARect>,
) -> (HashMap<SpatialId, Avalibility>, SweepAndPrune) {
    let mut avals = HashMap::<SpatialId, Avalibility>::new();
    let mut map = SweepAndPrune::new(Vec::new());
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
