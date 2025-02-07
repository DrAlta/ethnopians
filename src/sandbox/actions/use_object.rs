use bevy::prelude::*;
use qol::logy;

use crate::sandbox::{
    within_range,
    world::{Energy, Hp, Size, Type},
    EntityId, Item, Location, MAX_ENERGY,
};

use super::{PosibleActionsRequest, PosibleActionsResponce};

#[derive(Debug, PartialEq, PartialOrd, Clone)]
enum Command {
    RemoveAndAddToInvetory{
        remove:EntityId,
        inventory: EntityId,
        item: Item,
    },
    Heal{
        agent_id: EntityId,
        energy: i32,
        hp: i32,
    },
    Rest {
        agent_id: EntityId,
        amount: i32
    }
}


#[derive(Event, Debug)]
pub struct UseRequest {
    pub agent_id: EntityId,
    pub target_id: EntityId,
}

pub fn use_object_system(
    mut query: Query<(Entity, &Type, &Location, Option<&Size>, Option<&mut Energy>, Option<&mut Hp>)>,
    mut use_requests: EventReader<UseRequest>,
    mut posible_actions_requests: EventReader<PosibleActionsRequest>,
    mut posible_actions_responce: EventWriter<PosibleActionsResponce>,
    mut commands: Commands,
) {
    println!(
        "-----\nuse_system\n-----{:?} : {:?}",
        posible_actions_requests.len(),
        use_requests.len()
    );
    for PosibleActionsRequest {
        agent_id,
        target_id,
    } in posible_actions_requests.read()
    {
        println!("----------------------------------------------boo!");
        match use_object(&query, *agent_id, *target_id) {
            Ok(_) => {
                println!("sending response");
                posible_actions_responce.send(PosibleActionsResponce {
                    agent_id: *agent_id,
                    target_id: *target_id,
                    action_id: super::ActionId::UseObject,
                });
            }
            Err(err) => {
                logy!("trace", "{err}");
            }
        }
        {}
    }
    for UseRequest { agent_id, target_id } in use_requests.read()
    {
        match use_object(&query, *agent_id, *target_id) {
            Ok(command) => {
                match command {
                    Command::RemoveAndAddToInvetory { remove, inventory, item } => {
                        commands.entity(remove).despawn();
                        commands.spawn((
                            Type(item),
                            Location::Inventory(inventory)
                        ));
                    },
                    Command::Heal { agent_id, energy, hp } => {
                        if let Ok((_, _, _, _, energy_maybe, hp_maybe)) = query.get_mut(agent_id) {
                            if let Some(mut object_energy) = energy_maybe {
                                let Energy(x) = object_energy.as_mut();
                                *x += energy;
                            }
                            if let Some(mut object_hp) = hp_maybe {
                                let Hp(x) = object_hp.as_mut();
                                *x += hp;
                            }
                        }
                    },
                    Command::Rest { agent_id, amount } => {
                        if let Ok((_, _, _, _, energy_maybe, _)) = query.get_mut(agent_id) {
                            if let Some(mut object_energy) = energy_maybe {
                                let Energy(x) = object_energy.as_mut();
                                *x += amount;
                            }
                        }
                    },
                }
            }
            Err(err) => {
                logy!("trace", "{err}");
            }
        }
        {}
    }
}
fn use_object(
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
#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Component)]
    struct AgentIdObjectId(EntityId, EntityId);
    fn test_system(
        query: Query<(Entity, &AgentIdObjectId)>,
        mut events: EventWriter<PosibleActionsRequest>,
        mut commands: Commands,
    ) {
        for (id, &AgentIdObjectId(agent_id, target_id)) in &query {
            events.send(PosibleActionsRequest {
                agent_id,
                target_id,
            });
            commands.entity(id).despawn();
        }
    }

    #[test]
    pub fn no_agent_test() {
        std::env::set_var("RUST_BACKTRACE", "1");
        let mut app = App::new();
        app.add_event::<UseRequest>();
        app.add_event::<PosibleActionsRequest>();
        app.add_event::<PosibleActionsResponce>();
        app.add_systems(Update, (test_system, use_object_system).chain());
        let target_id = app.world_mut().spawn(Type(Item::Food)).id();

        app.world_mut()
            .spawn(AgentIdObjectId(Entity::from_raw(0), target_id));

        app.update();

        let response_events = app.world().resource::<Events<PosibleActionsResponce>>();
        let mut response_reader = response_events.get_cursor();
        assert!(response_reader.read(response_events).next().is_none());
    }

    #[test]
    pub fn someones_else_object_test() {
        std::env::set_var("RUST_BACKTRACE", "1");
        let mut app = App::new();
        app.add_event::<UseRequest>();
        app.add_event::<PosibleActionsRequest>();
        app.add_event::<PosibleActionsResponce>();
        app.add_systems(Update, (test_system, use_object_system).chain());
        let owner_id = app.world_mut().spawn(Type(Item::Food)).id();
        let target_id = app
            .world_mut()
            .spawn((Type(Item::Food), Location::Inventory(owner_id)))
            .id();
        let agent_id = app.world_mut().spawn(Type(Item::Agent)).id();

        app.world_mut().spawn(AgentIdObjectId(agent_id, target_id));

        app.update();

        let response_events = app.world().resource::<Events<PosibleActionsResponce>>();
        let mut response_reader = response_events.get_cursor();
        assert!(response_reader.read(response_events).next().is_none());
    }
    #[test]
    pub fn agent_in_another_world_test() {
        std::env::set_var("RUST_BACKTRACE", "1");
        let mut app = App::new();
        app.add_event::<UseRequest>();
        app.add_event::<PosibleActionsRequest>();
        app.add_event::<PosibleActionsResponce>();
        app.add_systems(Update, (test_system, use_object_system).chain());
        let world_id = app.world_mut().spawn(Type(Item::House)).id();
        let agent_id = app
            .world_mut()
            .spawn((Type(Item::Agent), Location::Inventory(world_id)))
            .id();
        let target_id = app
            .world_mut()
            .spawn((Type(Item::Veggie), Location::World { x: 1.0, y: 1.0 }))
            .id();

        app.world_mut().spawn(AgentIdObjectId(agent_id, target_id));

        app.update();

        let response_events = app.world().resource::<Events<PosibleActionsResponce>>();
        let mut response_reader = response_events.get_cursor();
        assert!(response_reader.read(response_events).next().is_none());
    }
    #[test]
    pub fn too_far_test() {
        std::env::set_var("RUST_BACKTRACE", "1");
        let mut app = App::new();
        app.add_event::<UseRequest>();
        app.add_event::<PosibleActionsRequest>();
        app.add_event::<PosibleActionsResponce>();
        app.add_systems(Update, (test_system, use_object_system).chain());
        let target_id = app
            .world_mut()
            .spawn((Type(Item::Veggie), Location::World { x: 0.0, y: 0.0 }))
            .id();
        let agent_id = app
            .world_mut()
            .spawn((Type(Item::Agent), Location::World { x: 0.0, y: 100.0 }))
            .id();

        app.world_mut().spawn(AgentIdObjectId(agent_id, target_id));

        app.update();

        let response_events = app.world().resource::<Events<PosibleActionsResponce>>();
        let mut response_reader = response_events.get_cursor();
        assert!(response_reader.read(response_events).next().is_none());
    }
    #[test]
    pub fn no_object_location_test() {
        std::env::set_var("RUST_BACKTRACE", "1");
        let mut app = App::new();
        app.add_event::<UseRequest>();
        app.add_event::<PosibleActionsRequest>();
        app.add_event::<PosibleActionsResponce>();
        app.add_systems(Update, (test_system, use_object_system).chain());
        let target_id = app.world_mut().spawn(Type(Item::Veggie)).id();
        let agent_id = app
            .world_mut()
            .spawn((Type(Item::Agent), Location::World { x: 0.0, y: 0.0 }))
            .id();

        app.world_mut().spawn(AgentIdObjectId(agent_id, target_id));

        app.update();

        let response_events = app.world().resource::<Events<PosibleActionsResponce>>();
        let mut response_reader = response_events.get_cursor();
        assert!(response_reader.read(response_events).next().is_none());
    }
    #[test]
    pub fn no_object_type_test() {
        std::env::set_var("RUST_BACKTRACE", "1");
        let mut app = App::new();
        app.add_event::<UseRequest>();
        app.add_event::<PosibleActionsRequest>();
        app.add_event::<PosibleActionsResponce>();
        app.add_systems(Update, (test_system, use_object_system).chain());
        let target_id = app
            .world_mut()
            .spawn(Location::World { x: 0.0, y: 0.0 })
            .id();
        let agent_id = app
            .world_mut()
            .spawn((Type(Item::Agent), Location::World { x: 0.0, y: 0.0 }))
            .id();

        app.world_mut().spawn(AgentIdObjectId(agent_id, target_id));

        app.update();

        let response_events = app.world().resource::<Events<PosibleActionsResponce>>();
        let mut response_reader = response_events.get_cursor();
        assert!(response_reader.read(response_events).next().is_none());
    }
    #[test]
    pub fn use_test() {
        //std::env::set_var("RUST_BACKTRACE", "1");
        let mut app = App::new();
        app.add_event::<UseRequest>();
        app.add_event::<PosibleActionsRequest>();
        app.add_event::<PosibleActionsResponce>();
        app.add_systems(Update, (test_system, use_object_system).chain());

        let agent_id = app
            .world_mut()
            .spawn((Type(Item::Agent), Location::World { x: 0.0, y: 0.0 }))
            .id();
        let target_id = app
            .world_mut()
            .spawn((Type(Item::Veggie), Location::Inventory(agent_id)))
            .id();

        app.world_mut().spawn(AgentIdObjectId(agent_id, target_id));

        app.update();

        let response_events = app.world().resource::<Events<PosibleActionsResponce>>();
        let mut response_reader = response_events.get_cursor();
        assert_eq!(
            response_reader.read(response_events).next(),
            Some(&PosibleActionsResponce {
                agent_id,
                target_id,
                action_id: crate::sandbox::ActionId::UseObject
            })
        );
    }
}
