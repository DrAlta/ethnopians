use bevy::prelude::*;

use crate::sandbox::{
    actions::use_object::Command,
    within_range,
    world::{Energy, Hp, Size, Type},
    EntityId, Item, Location, MAX_ENERGY,
};




pub fn use_object(
    query: &Query<(Entity, &Type, &Location, Option<&Size>, Option<&mut Energy>, Option<&mut Hp>)>,
    agent_id: EntityId,
    object_id: EntityId,
) -> Result<Command, String> {
    // get the agent
    let Ok((_, Type(Item::Agent), _, _, _, _)) = query.get(agent_id) else {
        return Err("Agent not found!".to_owned());
    };

    // get object's location and check if it is in the agents's invetory/
    // if not check if it's in range of the agent.
    match query.get(object_id) {
        // the object is in an inventory, so check if it's the agents' inventory
        Ok((_, _, Location::Inventory(inventory), _, _, _)) => {
            // is it the agents' inventory
            if inventory != &agent_id {
                return Err("Object in someone else's inventory".to_owned());
            }
        }
        // THe object is in the world, so check if it is in range of the agent
        Ok((
            _,
            _,
            Location::World {
                x: object_x,
                y: object_y,
            },
            _,
            _,
            _,
        )) => {
            // get the agent's location in the world.
            let Ok((
                _,
                _,
                Location::World {
                    x: agent_x,
                    y: agent_y,
                },
                _,
                _,
                _,
            )) = query.get(agent_id)
            else {
                return Err("Actor not in the world with object".to_owned());
            };
            // check is they are within range
            let (agent_center_x, agent_center_y) =
                if let Ok((_, _, _, Some(size), _, _)) = query.get(agent_id) {
                    (
                        agent_x + (size.width as f32 * 0.5),
                        agent_y + (size.height as f32 * 0.5),
                    )
                } else {
                    (*agent_x, *agent_y)
                };
            let (object_center_x, object_center_y) =
                if let Ok((_, _, _, Some(size), _, _)) = query.get(agent_id) {
                    (
                        object_x + (size.width as f32 * 0.5),
                        object_y + (size.height as f32 * 0.5),
                    )
                } else {
                    (*object_x, *object_y)
                };
            if within_range(
                agent_center_x,
                agent_center_y,
                object_center_x,
                object_center_y,
                20.0,
            ) {
                return Err("object is too far away!".to_owned());
            };
        }
        // there is no location recorded for the object
        Err(_) => {
            return Err("object's location not found!".to_owned());
        }
    }
    // get the object's type
    let Ok((_, Type(object), _, _, _, _)) = query.get(object_id) else {
        return Err("object's type not found!".to_owned());
    };
    // decide what to do based on the objects type
    match object {
        Item::Agent => todo!(),
        Item::Axe => todo!(),
        Item::Food => todo!(),
        Item::Stone => todo!(),
        Item::Stick => todo!(),
        Item::Wood => todo!(),
        // the objet was a house, agent will sleep in it to regain energy and maybe health
        Item::House => {
            let Ok((_, _, _, _, Some(Energy(energy)), _)) = query.get(agent_id) else {
                return Err("agent doesn't have energy".to_owned());
            };
            let excess = ((energy + 10) - MAX_ENERGY).max(0);
            let rest: i32 = 10 - excess;

            return Ok(if excess != 0 {
                Command::Heal {
                    agent_id,
                    energy: rest,
                    hp: excess,
                }
            } else {
                Command::Rest {
                    agent_id,
                    amount: rest,
                }
            });
        }
        Item::Tree => {
            let Some(_axe_idx) = query.iter().find_map(|(idx, Type(obj), _, _, _, _)| {
                let Ok((_, _, Location::Inventory(loc_id), _, _, _)) = query.get(idx) else {
                    return None;
                };
                if obj == &Item::Axe && agent_id == *loc_id {
                    Some(idx)
                } else {
                    None
                }
            }) else {
                return Err("Agent doesn't have an axe!".to_owned());
            };
            return Ok(Command::RemoveAndAddToInvetory { 
                remove: object_id, 
                inventory: agent_id, 
                item: Item::Wood });
        }
        Item::Veggie => {
            return Ok(Command::RemoveAndAddToInvetory { 
                remove: object_id, 
                inventory: agent_id, 
                item: Item::Food });
        }
    }
}
