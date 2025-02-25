use std::collections::{BTreeSet, HashMap};

use bevy::{
    ecs::event::EventWriter,
    prelude::{Commands, Query},
};

use crate::{util::AARect, Number};

use qol::logy;

use crate::{
    sandbox::{
        movement::{moveit, setup_avals_map, Collision, Prev, TravelCompleted},
        world::{Movement, Size},
        EntityId, Location,
    },
    Vec2,
};
/// I changed it to go thou the query and build a
/// Hashmap<EntityId, Vec2> of the normalized direction of traval
/// then just look inot that to see who collides with who
pub fn process_movement(
    mut query: Query<(EntityId, Option<&Movement>, Option<&mut Location>, &Size)>,
    mut collision_events: EventWriter<Collision>,
    mut travel_completed_events: EventWriter<TravelCompleted>,
    mut commands: Commands,
) {
    let max_step = 5.0;
    let time_step = 1.0;
/*
    #[cfg(feature = "move_history")]
    logy!("debug-process-movement", "Going tosaving histoy");
*/
    let normalize_dir_of_travel: HashMap<EntityId, (Vec2, Number)> = query
        .iter()
        .filter_map(|x| {
            let (id, Some(Movement { target, speed }), Some(&Location::World { x, y }), _) = x
            else {
                return None;
            };
            let dir = (target - &Vec2 { x, y }).normalize();

            Some((id, (dir, *speed)))
        })
        .collect();

    let number_of_substeps = query.iter().fold(1.0, |x, (_, movement_maybe, _, _)| {
        if let Some(Movement { target: _, speed }) = movement_maybe {
            let step_dist = speed * time_step;
            logy!(
                "debug-process-movement",
                "step_dist / max_step = {} / {} = {}",
                step_dist,
                max_step,
                step_dist / max_step
            );
            Number::max(x, (step_dist / max_step).ceil())
        } else {
            x
        }
    });
    let time_substep = time_step / number_of_substeps;

    let mut rearendings = HashMap::<EntityId, AARect>::new();
    let mut collisions: HashMap<EntityId, AARect> = query
        .iter()
        .filter_map(|(id, movement_maybe, location_maybe, size)| {
            match (movement_maybe, location_maybe) {
                (None, Some(Location::World { x, y })) => {
                    let entity = AARect {
                        min_x: *x,
                        min_y: *y,
                        width: size.width as Number,
                        height: size.height as Number,
                    };
                    Some((id, entity))
                }
                (_, _) => None,
            }
        })
        .collect();

    let mut collies = BTreeSet::new();
    let mut froms = HashMap::<EntityId, AARect>::new();
    //#[cfg(feature = "move_history")]
    //let mut history = Vec::new();
    let mut last_froms = HashMap::<EntityId, (Number, Number)>::new();
    for step_number in 1..(number_of_substeps as usize + 1) {
        logy!("debug-process-movement", "processing step {step_number}");
        let desired = query.iter().filter_map(
            |
                (
                    unit_id,
                    movement_maybe,
                    location_maybe,
                    _,
                )
            | {
                if let Some(Movement{ target: Vec2{x: tx, y: ty}, speed}) = movement_maybe {
                    if collisions.contains_key(&unit_id) || rearendings.contains_key(&unit_id) {
                    logy!("debug-process-movement", "this is an early out if this unit already has a collision which has been carried over since the last substep");
                        return None;
                    }
                    let Some(Location::World { x, y }) = location_maybe else {
                        logy!("debug-process-movement", "the unit doesn't have a location in the world");
                        return None;
                    };
                    let step_dist = speed * time_substep * step_number as Number;
                    let target_vec= Vec2{x: *tx, y: *ty};
                    let origin_vec = Vec2{x:*x, y:*y};

                    let delta = (target_vec - origin_vec).normalize() * step_dist;
                    if (target_vec - origin_vec).length_squared() < (step_dist * step_dist) + Number::EPSILON {
                        logy!("debug-process-movement", " the unit is moving more that the distance to the target so returning the target");
                        Some((unit_id.clone(), (target_vec.x, target_vec.y)))
                    } else {
                        logy!("debug-process-movement", "the unit is moving less that the distance to the target so returning the origin + (direction_of_motion * distance_traveled)");
                        let step = Vec2{x:*x, y:*y} + delta;
                        Some((unit_id.clone(), (step.x, step.y)))
                    }
                } else {
                    None
                }
            }
        )
        .collect();

        let (avals, map) = setup_avals_map(collisions, rearendings);
        let temp_collies;
        ([froms, collisions, rearendings], temp_collies) = if step_number == 1 {
            moveit(desired, avals, map, &query)
        } else {
            let prev = Previous {
                sizes: query
                    .iter()
                    .filter_map(|(id, _, _, Size { width, height })| {
                        Some((id, (*width as Number, *height as Number)))
                    })
                    .collect(),
                locations: &last_froms,
            };
            moveit(desired, avals, map, &prev)
        };
        temp_collies.into_iter().for_each(|(a,b)| {
            match (normalize_dir_of_travel.get(&a), normalize_dir_of_travel.get(&b)){
                (Some((a_dir, a_speed)), Some((b_dir, b_speed))) => {
                    match (a_speed.abs() < 0.0001, b_speed.abs() < 0.0001) {
                        (true, true) => {
                            collies.insert((a,b));
                            collies.insert((b,a));
                        },
                        (true, false) => {
                            //a is stiall do b ran into a
                            collies.insert((b,a));
                            collisions.remove(&a);
                            rearendings.remove(&a);
                        },
                        (false, true) => {
                            //b is still so a ran into b
                            collies.insert((a,b));
                            collisions.remove(&b);
                            rearendings.remove(&b);
                        },
                        (false, false) => {
                            let  dot = a_dir.dot(b_dir);

                            match(dot.abs() < 0.0001, a_speed.abs().total_cmp(&b_speed.abs()), (dot * a_speed).total_cmp(b_speed)){
                                // they aren't moving at right angle to each other and the component of A's speed in B's direction is less that B's speed
                                (false, _, std::cmp::Ordering::Less) |
                                // or they are moving at right able and A's speed is less that B's speed
                                (true, std::cmp::Ordering::Less, _) => {
                                   //b ran into a
                                   collies.insert((b,a));
                                   collisions.remove(&a);
                                   rearendings.remove(&a);
                                },
                                // they aren't moving at right angle to each other and the component of A's speed in B's direction is equal to B's speed
                                (false, _, std::cmp::Ordering::Equal) |
                                // or they are moving at right able and A's speed is equal B's speed
                                (true, std::cmp::Ordering::Equal, _) => {
                                    collies.insert((a,b));
                                    collies.insert((b,a));
                                },
                                // they aren't moving at right angle to each other and the component of A's speed in B's direction is greater that B's speed
                                (false, _, std::cmp::Ordering::Greater) |
                                // or they are moving at right able and A's speed is greater than B's speed
                                (true, std::cmp::Ordering::Greater, _) => {
                                    //a ran into b
                                    collies.insert((a,b));
                                    collisions.remove(&b);
                                    rearendings.remove(&b);
                                },
                            }
                        },
                    }
                },
                (Some(_), None) => {
                    collies.insert((a,b));
                },
                (None, Some(_)) => {
                    collies.insert((b,a));
                },
                (None, None)=> {
                    collies.insert((a,b));
                    collies.insert((b,a));
                }
            }
        });

        last_froms = froms
            .iter()
            .map(|(id, entity)| (id.clone(), (entity.get_min_x(), entity.get_min_y())))
            .collect();
        /*
        #[cfg(feature = "move_history")]
        history.push([froms.clone(), collisions.clone(), rearendings.clone()]);
        */
    }
    let mut moves = Vec::new();
    for (unit_id, entity) in froms {
        // moving entities to ther new locations
        let AARect { min_x, min_y, .. } = entity;
        let Some((x, y)) = query.get_location(unit_id) else {
            continue;
        };
        if (min_x - x).abs() > Number::EPSILON || (min_y - y).abs() > Number::EPSILON {
            moves.push((unit_id, (min_x, min_y)));
        }
    }
    for (id, (x, y)) in moves {
        // see if the entity reached it's destication
        if let Ok((_, Some(Movement { target, speed: _ }), _, _)) = query.get(id) {
            if (x - target.x).abs() <= 0.0001 && (y - target.y).abs() <= 0.0001 {
                // it reached it's destination so...
                // send the TravelComplated event
                travel_completed_events.send(TravelCompleted { entity_id: id });
                // remove the Movement component
                commands.entity(id).remove::<Movement>();
            }
        }

        let Ok((_, _, location_maybe, _)) = query.get_mut(id) else {
            continue;
        };
        let new_loc = Location::World { x, y };
        if let Some(mut location) = location_maybe {
            let loc = location.as_mut();
            *loc = new_loc;
        } else {
            commands.entity(id).insert(new_loc);
        }
    }
    logy!("trace", "{} collsions found", collies.len());
    for (agent_id, collider_id) in collies {
        collision_events.send(Collision {
            agent_id,
            collider_id,
        });
        // remove the Movement component
        commands.entity(agent_id).remove::<Movement>();
    }
}

struct Previous<'a> {
    pub sizes: HashMap<EntityId, (Number, Number)>,
    pub locations: &'a HashMap<EntityId, (Number, Number)>,
}
impl<'a> Prev for Previous<'a> {
    fn get_location(&self, id: EntityId) -> Option<(Number, Number)> {
        let (x, y) = self.locations.get(&id)?;
        Some((*x, *y))
    }

    fn get_size(&self, id: EntityId) -> Option<(Number, Number)> {
        let (w, h) = self.sizes.get(&id)?;
        Some((*w, *h))
    }
}
