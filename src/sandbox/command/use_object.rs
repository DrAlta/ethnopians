use bevy::prelude::*;

use crate::{mate::Agent, sandbox::{within_range, world::{Energy, Size, Type}, EntityId, Item, Location, MAX_ENERGY}};

use super::{Command, PosibleActionsResponce, PosibleActionsRequest};

#[derive(Event, Debug)]
pub struct UseRequest {
    pub agent_id: EntityId,
    pub target_id: EntityId,
}


fn use_object_system(
    mut query: Query<(
        Entity,
        &Type,
        &Location,
    )>,
    use_requests: EventReader<UseRequest>,
    posible_actions_requests: EventReader<PosibleActionsRequest>,
    mut posible_actions_responce: EventWriter<PosibleActionsResponce>,
) {
}
fn use_object(
    query: &Query<(
        Entity,
        &Type,
        &Location,
        &Size,
        Option<&Energy>,
    )>,
    agent_id: EntityId,
    object_id: EntityId,
) -> Option<Vec<Command>> {
    // get the agent
    let Ok((_, Type(Item::Agent), _, _, _)) = query.get(agent_id) else {
        return None//Return::ActionInvalid("Agent not found!".to_owned());
    };

    // get object's location and check if it is in the agents's invetory/
    // if not check if it's in range of the agent.
    match query.get(object_id) {
            // the object is in an inventory, so check if it's the agents' inventory
            Ok((_, _, Location::Inventory(inventory), _, _)) => {
                // is it the agents' inventory
                if inventory != &agent_id {
                    return None//Return::ActionInvalid("Object in someone else's inventory".to_owned());
                }
            }
            // THe object is in the world, so check if it is in range of the agent
            Ok((_, _, Location::World {
                x: object_x,
                y: object_y,
            }, _, _)) => {
                // get the agent's location in the world.
                let Ok((_, _, Location::World {
                    x: agent_x,
                    y: agent_y,
                }, _, _)) = query.get(agent_id)
                else {
                    return None//Return::ActionInvalid("Actor not in the world with object".to_owned());
                };
                // check is they are within range
                let (agent_center_x, agent_center_y) = if let Ok((_, _, _, size, _)) = query.get(agent_id)
                {
                    (agent_x + (size.width as f32 * 0.5), agent_y + (size.height as f32 * 0.5))
                } else {
                    (*agent_x, *agent_y)
                };
                let (object_center_x, object_center_y) =
                    if let Ok((_, _, _, size, _)) = query.get(agent_id) {
                        (object_x + (size.width as f32 * 0.5), object_y + (size.height as f32 * 0.5))
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
                    return None//Return::ActionInvalid("object is too far away!".to_owned());
                };
            }
            // there is no location recorded for the object
            Err(_) => {
                return None//Return::ActionInvalid("object's location not found!".to_owned());
            }
        }
        // get the object's type
        let Ok((_, Type(object), _, _, _)) = query.get(object_id) else {
            return None//Return::ActionInvalid("object's type not found!".to_owned());
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
                let Ok((_, _, _, _, Some(Energy(energy)))) = query.get(agent_id) else {
                    return None// Return::ActionInvalid("agent doesn't have energy".to_owned());
                };
                let excess = ((energy + 10) - MAX_ENERGY).max(0);
                let rest: i32 = 10 - excess;
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
                return Some(ret);
            }
            Item::Tree => {
                let Some(_axe_idx) = query.iter().find_map(|(idx, Type(obj), _, _, _)| {
                    let Ok((_,_,Location::Inventory(loc_id),_,_)) = query.get(idx) else{
                        return None;
                    };
                    if obj == &Item::Axe
                        && agent_id == *loc_id
                    {
                        Some(idx)
                    } else {
                        None
                    }
                }) else {
                    return None//Return::ActionInvalid("Agent doesn't have an axe!".to_owned());
                };
                return Some(vec![
                    Command::Remove(object_id),
                    Command::AddItem {
                        item: Item::Wood,
                        loc: Location::Inventory(agent_id),
                    },
                ]);
            }
            Item::Veggie => {
                return Some(vec![
                    Command::Remove(object_id),
                    Command::AddItem {
                        item: Item::Food,
                        loc: Location::Inventory(agent_id),
                    },
                ])
            }
        }
    }
#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use super::*;
    #[derive(Event)]
    struct AgentIdObjectId(EntityId, EntityId);

    fn no_agent_test_system(query: Query<(
        Entity,
        &Type,
        &Location,
        &Size,
        Option<&Energy>,
    )>,
    mut object_ids: EventReader<AgentIdObjectId>,
    ){
        for &AgentIdObjectId(agent_id, object_id) in object_ids.read() {
            let x = use_object(&query, agent_id, object_id);

            assert_eq!(x, None)
//            assert_eq!(x, Return::ActionInvalid("Agent not found!".into()))
        }

    }

    #[test]
    pub fn no_agent_test() {
        std::env::set_var("RUST_BACKTRACE", "1");
        let mut app = App::new();
        app.add_event::<AgentIdObjectId>();
        app.add_systems(Update, no_agent_test_system);
        let object_id = app
            .world_mut()
            .spawn(Type(Item::Food))
            .id();
        let mut events = app.world_mut().resource_mut::<Events<AgentIdObjectId>>();
        events.send(
            AgentIdObjectId(
                Entity::from_raw(0), 
                object_id
            )
        );
        app.update();

    }
    /*
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
    */
}
