use std::collections::HashMap;

use bevy::prelude::*;

use broad_phase::{AARect, Entity, AABB};
use qol::logy;

use crate::{
    sandbox::{world::{Movement, Size}, EntityId, Location},
    Vec2,
};

use super::{moveit, setup_avals_map, Prev};
pub fn process_movement(
    mut query: Query<(
        EntityId,
        Option<&Movement>, 
        Option<&mut Location>,
        &Size
    )>,
    mut commands: Commands,
) {
    let max_step = 5.0;
    let time_step = 1.0;


    #[cfg(feature = "move_history")]
    logy!("debug-process-movement", "Going tosaving histoy");

    let number_of_substeps = query.iter().fold(1_f32, |x, (_, movement_maybe, _, _)| {
        if let Some(Movement{ target, speed }) = movement_maybe {
            let step_dist = speed * time_step;
            logy!(
                "debug-process-movement",
                "step_dist / max_step = {} / {} = {}",
                step_dist,
                max_step,
                step_dist / max_step
            );
            x.max((step_dist / max_step).ceil())
        } else {
            x
        }
    });
    let time_substep = time_step / number_of_substeps;

    let mut rearendings = HashMap::<EntityId, Entity>::new();
    let mut collisions: HashMap::<EntityId, Entity> = query
        .iter()
        .filter_map(
            |(
                id, 
                movement_maybe, 
                location_maybe,
                size
            )| {
                match (movement_maybe, location_maybe) {
                    (None, Some(Location::World { x, y })) => {
                        let entity = Entity::AARect(AARect { min_x: *x, min_y: *y, width: size.width as f32, height: size.height as f32 });
                        Some((id, entity))
                    },
                    (_, _) => None,
                }
            }
        )
        .collect();

    let mut froms = HashMap::<EntityId, Entity>::new();
    #[allow(unused_mut)]
    let mut history = Vec::new();
    let mut last_froms = HashMap::<EntityId, (f32, f32)>::new();
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
                    let step_dist = speed * time_substep * step_number as f32;
                    let target_vec= Vec2{x: *tx, y: *ty};
                    let origin_vec = Vec2{x:*x, y:*y};

                    let delta = (target_vec - origin_vec).normalize() * step_dist;
                    if (target_vec - origin_vec).length_squared() < (step_dist * step_dist) + f32::EPSILON {
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
        [froms, collisions, rearendings] = if step_number == 1 {
            moveit(desired, avals, map, &query)
        } else {
            let prev = Previous {
                sizes: query
                    .iter()
                    .filter_map(
                        |(id, _, _, Size{ width, height })| 
                        Some((id, (*width as f32, *height  as f32)))
                    ).collect(),
                locations: &last_froms,
            };
            moveit(desired, avals, map, &prev)
        };

        last_froms = froms
            .iter()
            .map(|(id, entity)| (id.clone(), (entity.get_min_x(), entity.get_min_y())))
            .collect();
        #[cfg(feature = "move_history")]
        history.push([froms.clone(), collisions.clone(), rearendings.clone()]);
    }
    let mut moves = Vec::new();
    for (unit_id, entity) in froms {
        let Entity::AARect(AARect { min_x, min_y, .. }) = entity else {
            continue;
        };
        let Some((x, y)) = query.get_location(unit_id) else {
            continue;
        };
        if (min_x - x).abs() > f32::EPSILON || (min_y - y).abs() > f32::EPSILON {
            moves.push((
                unit_id,
                (min_x, min_y ),
            ));
        }
    }
    for (id, (x, y)) in moves {
        let Ok((_,_, location_maybe, _))= query.get_mut(id) else {
            continue
        };
        let new_loc = Location::World { x, y };
        if let Some(mut location) = location_maybe {
            let loc = location.as_mut();
            *loc = new_loc;
        } else {
            commands.entity(id).insert(new_loc);
        }

    }
}

struct Previous<'a> {
    pub sizes: HashMap<EntityId, (f32, f32)>,
    pub locations: &'a HashMap<EntityId, (f32, f32)>,
}
impl<'a> Prev for Previous<'a> {
    fn get_location(&self, id: EntityId) -> Option<(f32, f32)> {
        let (x, y) = self.locations.get(&id)?;
        Some((*x, *y))
    }

    fn get_size(&self, id: EntityId) -> Option<(f32, f32)> {
        let (w, h) = self.sizes.get(&id)?;
        Some((*w, *h))
    }
}
