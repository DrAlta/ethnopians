use crate::sandbox::{within_range, Item, Location, World, MAX_ENERGY};

use super::{Command, Return};

pub fn use_object(agent_idx: usize, object_idx:usize, world: &World) -> Return {
    // get the agent
    let Some(Item::Agent) = world.r#type.get(&agent_idx) else {
        return Return::ActionInvalid("Agent not found!".to_owned());
    };

    // get object's location and check if it is in the agents's invetory/
    // if not check if it's in range of the agent.
    match world.locations.get(&object_idx) {
        // the object is in an inventory, so check if it's the agents' inventory
        Some(Location::Inventory(inventory)) => {
        // is it the agents' inventory
        if inventory != &agent_idx {
                return Return::ActionInvalid("Object in someone else's inventory".to_owned())
            }
        },
        // THe object is in the world, so check if it is in range of the agent
        Some(Location::World { x, y }) => {
            // get the agent's location in the world.
            let Some(Location::World{x: agent_x, y: agent_y}) = world.locations.get(&agent_idx) else {
                return Return::ActionInvalid("Actor not in the world with object".to_owned())
            }; 
            // check is they are within range
            if within_range(*agent_x, *agent_y, *x, *y, 20.0) {
                return Return::ActionInvalid("object is too far away!".to_owned())
            };
        },
        // there is no location recorded for the object
        None => {
            return Return::ActionInvalid("object not found!".to_owned());
        },
    }
    // get the object's type
    let Some(object) = world.r#type.get(&object_idx) else {
        return Return::ActionInvalid("object not found!".to_owned());
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
            let Some(energy) = world.energy.get(&agent_idx) else {
                return Return::ActionInvalid("agent doesn't have energy".to_owned())
            };
            let excess = ((energy + 10) - MAX_ENERGY).max(0);
            let rest: i16= 10 - excess;
            let mut ret = vec![Command::Rest{agent_idx, ammount: rest}];

            if excess != 0 {
                ret.push(Command::Heal{agent_idx, ammount: excess})
            }
            return Return::Commands(ret)
        },
        Item::Tree => { 
            let Some(_axe_idx) = world.r#type.iter().find_map(|(idx, obj)|{
                if obj == &Item::Axe && Some(&Location::Inventory(agent_idx)) == world.locations.get(idx) { 
                    Some(idx)
                } else {
                    None
                }
            }) else {
                return Return::ActionInvalid("Agent doesn't have an axe!".to_owned());
            };
            return Return::Commands(vec![
                Command::Remove(object_idx),
                Command::AddItem{item:Item::Wood, loc:Location::Inventory(agent_idx)}
            ])
        },
        Item::Veggie => {
            return Return::Commands(vec![
                Command::Remove(object_idx),
                Command::AddItem{item:Item::Food, loc:Location::Inventory(agent_idx)}
            ])
        },
    }
}
