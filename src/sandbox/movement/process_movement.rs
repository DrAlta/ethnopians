use std::collections::HashMap;

use broad_phase::{AARect, Entity};

use crate::{sandbox::{Command, Location, ObjectId, Return, World}, Vec2};

use super::{moveit, setup_avals_map};

pub fn process_movement(max_step: f32, time_step: f32, world:&World) -> 
(Return<Command>, Vec<[HashMap::<ObjectId, Entity>;3]>) {
    #[cfg(feature = "move_history")]
    println!("Going tosaving histoy");

    let number_of_substeps = world.movement_iter().fold(
        1_f32,|x, (_,(_,speed))|{
            let step_dist = speed * time_step;
            x.max((step_dist / max_step).ceil())
        }
    );
    let time_substep = time_step / number_of_substeps;

    let mut rearendings = HashMap::<ObjectId, Entity>::new();
    let mut collisions = HashMap::<ObjectId, Entity>::new();
    let mut froms = HashMap::<ObjectId, Entity>::new();
    #[allow(unused_mut)]
    let mut history = Vec::new();
    for step_number in number_of_substeps as usize..(number_of_substeps as usize + 1){
        let desired = world.movement_iter().filter_map(
            |
                (
                    unit_id, 
                    (
                        (tx,ty), 
                        speed
                    )
                ) 
            | {
                if collisions.contains_key(unit_id) || rearendings.contains_key(unit_id) {
                    return None;
                }
                let Some(Location::World { x, y }) = world.get_location(unit_id) else {
                    return None;
                };
                let step_dist = speed * time_substep * step_number as f32;
                let target_vec= Vec2{x: *tx, y: *ty};
                let origin_vec = Vec2{x:*x, y:*y};
                if (target_vec - origin_vec).length_squared() < speed * time_substep {
                    Some((unit_id.clone(), (target_vec.x, target_vec.y)))
                } else {
                    let delta = (target_vec - origin_vec).normalize() * step_dist;
                    println!("{step_number}delta:{delta}");
                    let step = Vec2{x:*x, y:*y} + delta;
                    Some((unit_id.clone(), (step.x, step.y)))
                }
            }
        )
        .collect();
        let (avals, map) = setup_avals_map(collisions, rearendings);
    
        let x = moveit(desired, avals, map, world);
        #[cfg(feature = "move_history")]
        println!("saving histoy");
        #[cfg(feature = "move_history")]
        history.push(x.clone());

        [froms, collisions, rearendings] = x;

        
    }
    let mut commands = Vec::new();
    for (unit_id, entity) in rearendings {
        let Entity::AARect(AARect{ min_x, min_y, .. }) = entity else {
            continue;
        };
        let Some(Location::World { x, y }) = world.get_location(&unit_id) else {
            continue;
        };
        if (min_x - x).abs() > f32::EPSILON || (min_y - y).abs() > f32::EPSILON {
            commands.push(Command::SetLocation { agent_id: unit_id, loc: Location::World { x: min_x, y: min_y } });
        }
    }
    for (unit_id, entity) in froms {
        let Entity::AARect(AARect{ min_x, min_y, .. }) = entity else {
            continue;
        };
        let Some(Location::World { x, y }) = world.get_location(&unit_id) else {
            continue;
        };
        if (min_x - x).abs() > f32::EPSILON || (min_y - y).abs() > f32::EPSILON {
            commands.push(Command::SetLocation { agent_id: unit_id, loc: Location::World { x: min_x, y: min_y } });
        }
    }
    (Return::Commands(commands), history)

}

