use std::collections::{BTreeSet, HashMap};

use crate::{
    broadphase::{Broadphase, SpatialId},
    types::AARect,
};

use crate::sandbox::{movement::Prev, EntityId};

use super::Avalibility;

/// Handles cascading collisions between entities recursively.
///
/// # Parameters:
/// - `todo`: A set of `EntityId`s that need to be processed for collisions.
/// - `aval`: Mutable reference to the availability statuses.
/// - `map`: Mutable reference to the spatial map for collision detection.
/// - `prev`: Provides access to previous state information of entities.
/// - `collisions`: Mutable reference to the set recording collision pairs.
pub fn collision<T: Prev, B: Broadphase>(
    mut todo: BTreeSet<EntityId>,
    aval: &mut HashMap<SpatialId, Avalibility>,
    map: &mut B,
    prev: &T,
    collisions: &mut BTreeSet<(EntityId, EntityId)>,
) {
    loop {
        // Attempt to retrieve and remove the first entity from the 'todo' set.
        let Some(unit_id) = todo.pop_first() else {
            // Exit the loop when there are no more entities to process.
            return;
        };

        // Retrieve the previous location and size of the entity. Skip if unavailable.
        let Some((x, y)) = prev.get_location(unit_id) else {
            continue;
        };
        let Some(size) = prev.get_size(unit_id) else {
            continue;
        };

        // Query the map for entities overlapping with the current entity's previous location.
        let q = map.qurry(x.clone(), y.clone(), x + size.0, y + size.1);

        // Flag to determine if a 'RearEnded' status should be added.
        let mut add_rearended = false;

        // Check each overlapping spatial ID for potential collisions.
        for k in q {
            match aval.get_mut(&k) {
                // If the overlapping area is occupied by another entity's movement ('From').
                Some(cell @ Avalibility::From(_)) => {
                    let Avalibility::From(o) = cell else {
                        continue;
                    };
                    // If the occupying entity is the same, prepare to add a 'RearEnded' status.
                    if o == &unit_id {
                        add_rearended = true;
                    }
                    let o2 = o.clone();

                    // Record the collision between the entities.
                    let min_id = o2.min(unit_id);
                    let max_id = o2.max(unit_id);
                    collisions.insert((min_id, max_id));

                    // Update the availability to 'Collision' and add the source entity to the 'todo' set for further processing.
                    if let Avalibility::From(source_object) =
                        std::mem::replace(cell, Avalibility::Collision(o2))
                    {
                        todo.insert(source_object);
                    };
                    continue;
                }
                // Ignore if the area already has a 'Collision' or 'RearEnded' status.
                Some(Avalibility::Collision(_)) | Some(Avalibility::RearEnded(_)) => (),
                // No availability status; do nothing.
                None => (),
            }
        }

        // If necessary, place a 'RearEnded' status at the entity's previous location.
        if add_rearended {
            let k = map.insert(AARect::new(x.clone(), y.clone(), size.0, size.1));
            aval.insert(k, Avalibility::RearEnded(unit_id.clone()));
        }
    }
}
