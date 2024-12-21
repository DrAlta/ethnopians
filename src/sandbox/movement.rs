use std::collections::{BTreeSet, HashMap};

use broad_phase::{AARect, Entity, EntityId, SpatialBloom};

use super::{Location, ObjectId, World};


#[allow(dead_code)]
pub enum Avalibility {
    From(ObjectId),
    Collision(ObjectId),
    RearEnded(ObjectId),
 }
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
#[allow(dead_code)]
pub fn moveit(desired: HashMap<ObjectId, (f32, f32)>, world: &World) {
    let mut aval = HashMap::<EntityId, Avalibility>::new();
    let mut map = SpatialBloom::new(
        10.0,
        10.0,
        Vec::new(),
    ).unwrap();
    for (unit_id, destination) in desired {
        let Some(size) = world.get_size(&unit_id) else {
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
            match aval.get_mut(&k) {
                Some(cell@ Avalibility::From(_)) => {
                    let Avalibility::From(o) = cell else {
                        continue
                    };
                    let o2 = o.clone();
                    if let Avalibility::From(source_object) = std::mem::replace(cell, Avalibility::Collision(o2)) {
                        collision(BTreeSet::from([source_object.clone()]), &mut aval, &mut map, &world);
                    };
                    let new_cell_id = map.insert(Entity::AARect(AARect::new(
                        destination.0, 
                        destination.1,
                        size.0,
                        size.1, 
                    )));
                    aval.insert(new_cell_id, Avalibility::Collision(unit_id.clone()));
                    blocked = true;
                },
                Some(Avalibility::Collision(_)) => {
                    blocked = true;
                },
                Some(Avalibility::RearEnded(_)) => {
                    blocked = true;
                },
                None => ()
            }
        }
        if let Some(Location::World { x, y }) =world.get_location(&unit_id){
            let dest_aval;
            if blocked {
                dest_aval = Avalibility::Collision(unit_id.clone());
                let collision_cell_id = map.insert(Entity::AARect(AARect::new(*x, *y, size.0, size.1)));
                aval.insert(collision_cell_id, Avalibility::Collision(unit_id));
            } else {
                dest_aval = Avalibility::From(unit_id);
            }
            let dest_cell_id = map.insert(Entity::AARect(AARect::new(
                destination.0, 
                destination.1, 
                destination.0 + size.0, 
                destination.1 + size.1
            )));
            aval.insert(dest_cell_id, dest_aval);

        }
    }
 }

 fn collision(mut todo: BTreeSet<ObjectId>, aval: &mut HashMap::<EntityId, Avalibility>, map: &mut SpatialBloom, world: &World) {
    loop {

        let Some(unit_id) = todo.pop_first() else {
            return;
        };
        let Some(Location::World { x, y }) = world.get_location(&unit_id) else {
            continue;
        };
        let Some(size) = world.get_size(&unit_id) else {
            continue;
        };
        let q = map.qurry(
            x.clone(), 
            y.clone(),
            x + size.0, 
            y + size.1, 
        );
        let mut add_rearended = false;
        for k in q {
            match aval.get_mut(&k) {
                Some(cell@ Avalibility::From(_)) => {
                    let Avalibility::From(o) = cell else {
                        continue;
                    };
                    if o == &unit_id {
                        add_rearended = true;
                    }
                    let o2 = o.clone();
                    if let Avalibility::From(source_object) = std::mem::replace(cell, Avalibility::Collision(o2)) {
                        todo.insert(source_object);
                    };
                    continue
                },
                Some(Avalibility::Collision(_)) => (),
                Some(Avalibility::RearEnded(_)) => (),
                None => (),
            }
        }
        if add_rearended {
            let k = map.insert(Entity::AARect(AARect::new(x.clone(), y.clone(), size.0, size.1)));
            aval.insert(
                k,
                Avalibility::RearEnded(unit_id.clone())
            );
        }
    }
 }