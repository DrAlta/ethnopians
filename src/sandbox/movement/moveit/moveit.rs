use std::collections::{BTreeSet, HashMap};

use broad_phase::{AARect, Entity, EntityId, SpatialBloom};

use crate::sandbox::ObjectId;

use super::{collision, Avalibility, Prev};

//   1 2
// C R R
// 1 1 2
//
// add rearended: BTreeSet<ObjectId, ObjectId>;
// `match RearEnded(obstacle_id) => {rearended.insert((unit_id, obstacle_id))}`
//
/// need to change map.get() to take a rectacle and return all the overlaping rects in it
/// then when you detect a collicon add the a new `Avalibility::Collision(colliding_object)`
/// for when it wanted to move and a `Avalibility::RearEnded` for the unit's corrent rectangle  
pub fn moveit<T: Prev>(
    desired: HashMap<ObjectId, (f32, f32)>, 
    mut avals: HashMap::<EntityId, Avalibility>, 
    mut map: SpatialBloom,
    prev: &T,
) -> [HashMap<ObjectId, Entity>;3] {
    for (unit_id, destination) in desired {
        let Some(size) = prev.get_size(&unit_id) else {
            continue;
        };
        let q = map.qurry(
            destination.0,
            destination.1,
            destination.0 + size.0,
            destination.1 + size.1,
        );
        let mut blocked = false;
        for k in q {
            match avals.get_mut(&k) {
                Some(cell @ Avalibility::From(_)) => {
                    let Avalibility::From(o) = cell else { continue };
                    let o2 = o.clone();
                    if let Avalibility::From(source_object) =
                        std::mem::replace(cell, Avalibility::Collision(o2))
                    {
                        collision(
                            BTreeSet::from([source_object.clone()]),
                            &mut avals,
                            &mut map,
                            prev,
                        );
                    };
                    let new_cell_id = map.insert(Entity::AARect(AARect::new(
                        destination.0,
                        destination.1,
                        size.0,
                        size.1,
                    )));
                    avals.insert(new_cell_id, Avalibility::Collision(unit_id.clone()));
                    blocked = true;
                }
                Some(Avalibility::Collision(_)) => {
                    blocked = true;
                }
                Some(Avalibility::RearEnded(_)) => {
                    blocked = true;
                }
                None => (),
            }
        }
        if let Some(( x, y )) = prev.get_location(&unit_id) {
            let dest_aval;
            if blocked {
                dest_aval = Avalibility::Collision(unit_id.clone());
                let collision_cell_id =
                    map.insert(Entity::AARect(AARect::new(x, y, size.0, size.1)));
                avals.insert(collision_cell_id, Avalibility::Collision(unit_id));
            } else {
                dest_aval = Avalibility::From(unit_id);
            }
            let dest_cell_id = map.insert(Entity::AARect(AARect::new(
                destination.0,
                destination.1,
                size.0,
                size.1,
            )));
            avals.insert(dest_cell_id, dest_aval);
        }
    }
    let mut from= HashMap::new();
    let mut collision= HashMap::new();
    let mut rearended= HashMap::new();
    for (id, avalibity) in avals {
        match avalibity {
            Avalibility::From(unit_id) => {
                if let Some(entity) = map.get_entity(&id) {
                    from.insert(unit_id, entity.clone());
                }
            },
            Avalibility::Collision(unit_id) => {
                if let Some(entity) = map.get_entity(&id) {
                    collision.insert(unit_id, entity.clone());
                }
            },
            Avalibility::RearEnded(unit_id) => {
                if let Some(entity) = map.get_entity(&id) {
                    rearended.insert(unit_id, entity.clone());
                }
            },
        }
    }
    [from, collision, rearended]
}