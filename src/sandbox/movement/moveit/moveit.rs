use std::collections::{BTreeSet, HashMap};

use crate::{
    broadphase::{Broadphase, SpatialId},
    types::AARect,
    Number,
};
use qol::logy;

use crate::sandbox::EntityId;

use super::{collision, Avalibility, Prev};
//   1 2
// C R R
// 1 1 2
//
// add rearended: BTreeSet<EntityId, EntityId>;
// `match RearEnded(obstacle_id) => {rearended.insert((unit_id, obstacle_id))}`
//
/// need to change map.get() to take a rectacle and return all the overlaping rects in it
/// then when you detect a collicon add the a new `Avalibility::Collision(colliding_object)`
/// for when it wanted to move and a `Avalibility::RearEnded` for the unit's corrent rectangle  

/// Moves entities to their desired positions while handling collisions and updating availability.
///
/// # Parameters:
/// - `desired`: A map of entities to their desired (x, y) positions.
/// - `avals`: Current availability statuses for spatial IDs.
/// - `map`: Spatial map using the Sweep and Prune algorithm for efficient collision detection.
/// - `prev`: An object implementing the `Prev` trait, providing previous states of entities.
///
/// # Returns:
/// - An array of three `HashMap`s containing updated positions categorized as:
///   - `[0]`: Entities that successfully moved (`from`).
///   - `[1]`: Entities involved in collisions.
///   - `[2]`: Entities that were rear-ended.
/// - A Vec of tuples of `EntityId`s representing entities involved in collisions.
pub fn moveit<T: Prev, B: Broadphase<SpatialId>>(
    desired: HashMap<EntityId, (Number, Number)>,
    mut avals: HashMap<SpatialId, Avalibility>,
    mut map: B,
    prev: &T,
) -> (
    [HashMap<EntityId, AARect>; 3],
    BTreeSet<(EntityId, EntityId)>,
) {
    // Set to keep track of unique collision pairs between entities.
    let mut collisions = BTreeSet::new();

    // Iterate over each entity and its desired destination.
    for (unit_id, destination) in desired {
        // Retrieve the size of the entity from previous state. Skip if size is unknown.
        let Some(size) = prev.get_size(unit_id) else {
            continue;
        };

        // Query the spatial map for any entities overlapping with the desired destination rectangle.
        let q = map.qurry(
            destination.0,
            destination.1,
            destination.0 + size.0,
            destination.1 + size.1,
        );

        // Flag to indicate if the entity's movement is blocked by a collision.
        let mut blocked = false;

        // Check each overlapping spatial ID (potential collision).
        for k in q {
            match avals.get_mut(&k) {
                Some(cell @ Avalibility::From(_)) => {
                    // If the overlapping area is occupied by another entity's movement ('From').

                    // Extract the occupying entity's ID.
                    let Avalibility::From(o) = cell else { continue };
                    let o2 = o.clone();
                    // Replace the availability status to 'Collision' for both entities.
                    if let Avalibility::From(source_object) =
                        std::mem::replace(cell, Avalibility::Collision(o2.clone()))
                    {
                        // Record the collision between the two entities.
                        let min_id = source_object.min(unit_id);
                        let max_id = source_object.max(unit_id);
                        collisions.insert((min_id, max_id));

                        // Recursively handle further collisions caused by this update.
                        collision(
                            BTreeSet::from([source_object.clone()]),
                            &mut avals,
                            &mut map,
                            prev,
                            &mut collisions,
                        );
                    };

                    // Place a 'RearEnded' status at the previous location of the occupying entity.
                    if let Some((x, y)) = prev.get_location(o2) {
                        logy!(
                            "trace-moveit",
                            "putting Rearended in at the original location"
                        );
                        let rearend_cell_id = map.insert(AARect::from_min_w_h(x, y, size.0, size.1));
                        avals.insert(rearend_cell_id, Avalibility::RearEnded(o2));
                    }

                    // Mark the desired destination as having a collision.
                    let new_cell_id =
                        map.insert(AARect::from_min_w_h(destination.0, destination.1, size.0, size.1));
                    avals.insert(new_cell_id, Avalibility::Collision(unit_id.clone()));
                    blocked = true;
                }
                // If the area already has a collision or rear-ended status, movement is blocked.
                Some(Avalibility::Collision(other_id)) | Some(Avalibility::RearEnded(other_id)) => {
                    // Record the collision between the two entities.
                    let min_id = unit_id.min(*other_id);
                    let max_id = unit_id.max(*other_id);
                    collisions.insert((min_id, max_id));

                    blocked = true;
                }
                // No availability status; proceed without blocking.
                None => (),
            }
        }

        // Determine the availability status based on collision checks.
        let dest_aval;
        if blocked {
            logy!("trace-moveit", "the unit colided with something");
            // Mark the destination as a collision for the current entity.
            dest_aval = Avalibility::Collision(unit_id.clone());

            // Place a 'RearEnded' status at the entity's current location.
            if let Some((x, y)) = prev.get_location(unit_id) {
                logy!(
                    "trace-moveit",
                    "putting Rearended in at the original location"
                );
                let rearend_cell_id = map.insert(AARect::from_min_w_h(x, y, size.0, size.1));
                avals.insert(rearend_cell_id, Avalibility::RearEnded(unit_id));
            }
        } else {
            // Mark the destination as successfully occupied by the entity.
            dest_aval = Avalibility::From(unit_id);
        }

        // Insert the availability status into the map at the desired destination
        let dest_cell_id = map.insert(AARect::from_min_w_h(destination.0, destination.1, size.0, size.1));
        avals.insert(dest_cell_id, dest_aval);
    }

    // Create HashMaps to store entities based on their movement outcomes.
    let mut from = HashMap::new();
    let mut collision = HashMap::new();
    let mut rearended = HashMap::new();

    // Populate the HashMaps by iterating through the availability statuses.
    for (id, avalibity) in avals {
        match avalibity {
            // Entities that moved successfully to their destinations.
            Avalibility::From(unit_id) => {
                if let Some(entity) = map.get_entity(&id) {
                    from.insert(unit_id, entity.clone());
                }
            }
            // Entities that experienced collisions.
            Avalibility::Collision(unit_id) => {
                if let Some(entity) = map.get_entity(&id) {
                    collision.insert(unit_id, entity.clone());
                }
            }
            // Entities that were rear-ended (blocked from moving).
            Avalibility::RearEnded(unit_id) => {
                if let Some(entity) = map.get_entity(&id) {
                    rearended.insert(unit_id, entity.clone());
                }
            }
        }
    }

    // Return the categorized movement results and the collision pairs.
    ([from, collision, rearended], collisions)
}
