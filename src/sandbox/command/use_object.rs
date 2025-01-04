use crate::sandbox::{within_range, Item, Location, ObjectId, World, MAX_ENERGY};

use super::super::{Return, UseObject};
use super::Command;

impl UseObject<Command> for Command {
    fn use_object(agent_id: ObjectId, object_id: ObjectId, world: &World) -> Return<Command> {
        // get the agent
        let Some(Item::Agent) = world.get_type(&agent_id) else {
            return Return::ActionInvalid("Agent not found!".to_owned());
        };

        // get object's location and check if it is in the agents's invetory/
        // if not check if it's in range of the agent.
        match world.get_location(&object_id) {
            // the object is in an inventory, so check if it's the agents' inventory
            Some(Location::Inventory(inventory)) => {
                // is it the agents' inventory
                if inventory != &agent_id {
                    return Return::ActionInvalid("Object in someone else's inventory".to_owned());
                }
            }
            // THe object is in the world, so check if it is in range of the agent
            Some(Location::World {
                x: object_x,
                y: object_y,
            }) => {
                // get the agent's location in the world.
                let Some(Location::World {
                    x: agent_x,
                    y: agent_y,
                }) = world.get_location(&agent_id)
                else {
                    return Return::ActionInvalid("Actor not in the world with object".to_owned());
                };
                // check is they are within range
                let (agent_center_x, agent_center_y) = if let Some(size) = world.get_size(&agent_id)
                {
                    (agent_x + (size.0 * 0.5), agent_y + (size.1 * 0.5))
                } else {
                    (*agent_x, *agent_y)
                };
                let (object_center_x, object_center_y) =
                    if let Some(size) = world.get_size(&agent_id) {
                        (object_x + (size.0 * 0.5), object_y + (size.1 * 0.5))
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
                    return Return::ActionInvalid("object is too far away!".to_owned());
                };
            }
            // there is no location recorded for the object
            None => {
                return Return::ActionInvalid("object's location not found!".to_owned());
            }
        }
        // get the object's type
        let Some(object) = world.get_type(&object_id) else {
            return Return::ActionInvalid("object's type not found!".to_owned());
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
                let Some(energy) = world.get_energy(&agent_id) else {
                    return Return::ActionInvalid("agent doesn't have energy".to_owned());
                };
                let excess = ((energy + 10) - MAX_ENERGY).max(0);
                let rest: i16 = 10 - excess;
                let mut ret = vec![Command::Rest {
                    agent_id,
                    ammount: rest,
                }];

                if excess != 0 {
                    ret.push(Command::Heal {
                        agent_id,
                        ammount: excess,
                    })
                }
                return Return::Commands(ret);
            }
            Item::Tree => {
                let Some(_axe_idx) = world.type_iter().find_map(|(idx, obj)| {
                    if obj == &Item::Axe
                        && Some(&Location::Inventory(agent_id)) == world.get_location(idx)
                    {
                        Some(idx)
                    } else {
                        None
                    }
                }) else {
                    return Return::ActionInvalid("Agent doesn't have an axe!".to_owned());
                };
                return Return::Commands(vec![
                    Command::Remove(object_id),
                    Command::AddItem {
                        item: Item::Wood,
                        loc: Location::Inventory(agent_id),
                    },
                ]);
            }
            Item::Veggie => {
                return Return::Commands(vec![
                    Command::Remove(object_id),
                    Command::AddItem {
                        item: Item::Food,
                        loc: Location::Inventory(agent_id),
                    },
                ])
            }
        }
    }
}
#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::sandbox::interaction::get_interactions;

    use super::*;

    #[test]
    pub fn no_agent_test() {
        let acts = get_interactions::<Command>();
        let world = World::new(
            HashMap::new(),
            HashMap::new(),
            HashMap::new(),
            HashMap::new(),
            HashMap::from([(0, Item::Food)]),
            HashMap::new(),
        );
        let x = (acts[0].act)(42, 0, &world);
        assert_eq!(x, Return::ActionInvalid("Agent not found!".into()))
    }
    #[test]
    pub fn someones_else_object_test() {
        let acts = get_interactions::<Command>();
        let world = World::new(
            HashMap::from([(42, Location::Inventory(2))]),
            HashMap::new(),
            HashMap::new(),
            HashMap::new(),
            HashMap::from([(0, Item::Agent)]),
            HashMap::new(),
        );
        let x = (acts[0].act)(0, 42, &world);
        assert_eq!(
            x,
            Return::ActionInvalid("Object in someone else's inventory".into())
        )
    }
    #[test]
    pub fn agent_in_another_world_test() {
        let acts = get_interactions::<Command>();
        let world = World::new(
            HashMap::from([
                (0, Location::Inventory(2)),
                (42, Location::World { x: 1.0, y: 1.0 }),
            ]),
            HashMap::new(),
            HashMap::new(),
            HashMap::new(),
            HashMap::from([(0, Item::Agent)]),
            HashMap::new(),
        );
        let x = (acts[0].act)(0, 42, &world);
        assert_eq!(
            x,
            Return::ActionInvalid("Actor not in the world with object".into())
        )
    }
    #[test]
    pub fn too_far_test() {
        let acts = get_interactions::<Command>();
        let world = World::new(
            HashMap::from([
                (0, Location::World { x: 0.0, y: 0.0 }),
                (42, Location::World { x: 0.0, y: 100.0 }),
            ]),
            HashMap::new(),
            HashMap::new(),
            HashMap::new(),
            HashMap::from([(0, Item::Agent)]),
            HashMap::new(),
        );
        let x = (acts[0].act)(0, 42, &world);
        assert_eq!(x, Return::ActionInvalid("object is too far away!".into()))
    }
    #[test]
    pub fn no_object_location_test() {
        let acts = get_interactions::<Command>();
        let world = World::new(
            HashMap::from([(0, Location::World { x: 0.0, y: 0.0 })]),
            HashMap::new(),
            HashMap::new(),
            HashMap::new(),
            HashMap::from([(0, Item::Agent)]),
            HashMap::new(),
        );
        let x = (acts[0].act)(0, 42, &world);
        assert_eq!(
            x,
            Return::ActionInvalid("object's location not found!".into())
        )
    }
    #[test]
    pub fn no_object_type_test() {
        let acts = get_interactions::<Command>();
        let world = World::new(
            HashMap::from([
                (0, Location::World { x: 0.0, y: 0.0 }),
                (42, Location::World { x: 0.0, y: 0.0 }),
            ]),
            HashMap::new(),
            HashMap::new(),
            HashMap::new(),
            HashMap::from([(0, Item::Agent)]),
            HashMap::new(),
        );
        let x = (acts[0].act)(0, 42, &world);
        assert_eq!(x, Return::ActionInvalid("object's type not found!".into()))
    }
}
