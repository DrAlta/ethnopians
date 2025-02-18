use std::collections::{BTreeSet, HashMap};

use bevy::{ecs::event::EventWriter, prelude::{Commands, Query}};

use crate::{util::AARect, Number};

use qol::logy;

use crate::{
    sandbox::{
        world::{Movement, Size},
        movement::{moveit, setup_avals_map, Collision, Prev, TravelCompleted},
        EntityId, Location,
    },
    Vec2,
};

pub fn process_movement(
    mut query: Query<(EntityId, Option<&Movement>, Option<&mut Location>, &Size)>,
    mut collision_events: EventWriter<Collision>,
    mut travel_completed_events: EventWriter<TravelCompleted>,
    mut commands: Commands,
) {
    let max_step = 5.0;
    let time_step = 1.0;

    #[cfg(feature = "move_history")]
    logy!("debug-process-movement", "Going tosaving histoy");

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
    #[allow(unused_mut)]
    let mut history = Vec::new();
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
        let mut temp_collies;
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
        collies.append(&mut temp_collies);

        last_froms = froms
            .iter()
            .map(|(id, entity)| (id.clone(), (entity.get_min_x(), entity.get_min_y())))
            .collect();
        #[cfg(feature = "move_history")]
        history.push([froms.clone(), collisions.clone(), rearendings.clone()]);
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
            println!("{}:{} == {}", x,y,target);
            if (x - target.x).abs() <= 0.0001 && (y - target.y).abs() <= 0.0001 {
                // it reached it's destination so...
                // send the TravelComplated event
                travel_completed_events.send(TravelCompleted{ entity_id: id });
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
    for (min_id, max_id) in collies {
        
        match (query.get(min_id), query.get(max_id) ){
            (
                Ok((
                    _,
                    Some(Movement { target:min_target, speed: min_speed}), 
                    Some(Location::World { x: min_x, y: min_y }),
                    _
                )), 
                Ok((
                    _,
                    Some(Movement { target: max_target, speed: max_speed}), 
                    Some(Location::World { x: max_x, y: max_y }),
                    _,
                ))
            ) => {
                let min_dir = (min_target - &Vec2{x: *min_x, y: *min_y}).normalize();
                let max_dir = (max_target - &Vec2{x: *max_x, y: *max_y}).normalize();

                let mins_compenent_along_max = max_dir.dot(min_dir) * min_speed;
                if mins_compenent_along_max < *max_speed {
                    collision_events.send(Collision{ agent_id: max_id, collider_id: min_id });
                    // remove the Movement component
                    commands.entity(max_id).remove::<Movement>();    
                }

                let maxs_compenent_along_min = min_dir.dot(max_dir) * max_speed;
                if maxs_compenent_along_min < *min_speed {
                    collision_events.send(Collision{ agent_id: min_id, collider_id: max_id });
                    // remove the Movement component
                    commands.entity(min_id).remove::<Movement>();    
                }
            },
            _ => {
                collision_events.send(Collision{ agent_id: min_id, collider_id: max_id });
                collision_events.send(Collision{ collider_id: min_id, agent_id: max_id });
                // remove the Movement component
                commands.entity(min_id).remove::<Movement>();
                commands.entity(max_id).remove::<Movement>();
            }
        }

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
