use std::collections::BTreeSet;

use bevy::prelude::*;

use crate::{
    sandbox::{
        change_request::Changes,
        within_range,
        world::{Size, Type},
        EntityId, Item, Location,
    },
    Number,
};

pub fn use_on(
    query: &Query<(Entity, &Type, &Location, Option<&Size>)>,
    agent_id: EntityId,
    tool_id: EntityId,
    target_id: EntityId,
) -> Result<(BTreeSet<EntityId>, Vec<Changes>), String> {
    // get the agent
    let Ok((_, Type(Item::Agent), _, _)) = query.get(agent_id) else {
        return Err("Agent not found!".to_owned());
    };

    let tool_type = match query.get(tool_id) {
        // the tool is in an inventory, so check if it's the agents' inventory
        Ok((_, Type(tool_type), Location::Inventory(inventory), _)) => {
            // is it the agents' inventory
            if inventory != &agent_id {
                return Err("Object in someone else's inventory".to_owned());
            }
            tool_type
        }
        // THe target is in the world, so check if it is in range of the agent
        Ok((
            _,
            Type(tool_type),
            Location::World {
                x: tool_x,
                y: tool_y,
            },
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
            )) = query.get(agent_id)
            else {
                return Err("Actor not in the world with tool".to_owned());
            };
            // check is they are within range
            let (agent_center_x, agent_center_y) =
                if let Ok((_, _, _, Some(size))) = query.get(agent_id) {
                    (
                        agent_x + (Into::<Number>::into(size.width) * Number::HALF),
                        agent_y + (Into::<Number>::into(size.height) * Number::HALF),
                    )
                } else {
                    (*agent_x, *agent_y)
                };
            let (tool_center_x, tool_center_y) =
                if let Ok((_, _, _, Some(size))) = query.get(agent_id) {
                    (
                        tool_x + (Into::<Number>::into(size.width) * Number::HALF),
                        tool_y + (Into::<Number>::into(size.height) * Number::HALF),
                    )
                } else {
                    (*tool_x, *tool_y)
                };
            if within_range(
                agent_center_x,
                agent_center_y,
                tool_center_x,
                tool_center_y,
                Into::<Number>::into(20.0),
            ) {
                return Err("tool is too far away!".to_owned());
            };
            tool_type
        }
        // there is no location recorded for the tool
        Err(_) => {
            return Err("tools's location not found!".to_owned());
        }
    };

    let target_type = match query.get(target_id) {
        // the tool is in an inventory, so check if it's the agents' inventory
        Ok((_, Type(target_type), Location::Inventory(inventory), _)) => {
            // is it the agents' inventory
            if inventory != &agent_id {
                return Err("Object in someone else's inventory".to_owned());
            }
            target_type
        }
        // THe object is in the world, so check if it is in range of the agent
        Ok((
            _,
            Type(target_type),
            Location::World {
                x: target_x,
                y: target_y,
            },
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
            )) = query.get(agent_id)
            else {
                return Err("Actor not in the world with target".to_owned());
            };
            // check is they are within range
            let (agent_center_x, agent_center_y) =
                if let Ok((_, _, _, Some(size))) = query.get(agent_id) {
                    (
                        agent_x + (Into::<Number>::into(size.width) * Number::HALF),
                        agent_y + (Into::<Number>::into(size.height) * Number::HALF),
                    )
                } else {
                    (*agent_x, *agent_y)
                };
            let (target_center_x, target_center_y) =
                if let Ok((_, _, _, Some(size))) = query.get(agent_id) {
                    (
                        target_x + (Into::<Number>::into(size.width) * Number::HALF),
                        target_y + (Into::<Number>::into(size.height) * Number::HALF),
                    )
                } else {
                    (*target_x, *target_y)
                };
            if within_range(
                agent_center_x,
                agent_center_y,
                target_center_x,
                target_center_y,
                Into::<Number>::into(20.0),
            ) {
                return Err("target is too far away!".to_owned());
            };
            target_type
        }
        // there is no location recorded for the target
        Err(_) => {
            return Err("target's location not found!".to_owned());
        }
    };
    // decide what to do based on the targets type
    match target_type {
        Item::Agent => todo!(),
        Item::Axe => todo!(),
        Item::Food => todo!(),
        Item::Knife => {
            // I think using the Stick on the knife is better but we'll allow this too for now.
            if tool_type != &Item::Stick {
                return Err("Only a Stick can be used on a Knife!".to_owned());
            };
            make_axe(agent_id, target_id, tool_id)
        }
        Item::Seed => todo!(),
        Item::Stone => {
            if tool_type != &Item::Stone {
                return Err("Only a Stone can be used on a Stone!".to_owned());
            };
            let changes = vec![
                Changes::Despawn(target_id),
                Changes::Despawn(tool_id),
                Changes::SpawnLocationType {
                    location: Location::Inventory(agent_id),
                    tyep: Item::Knife,
                },
            ];
            return Ok((BTreeSet::from([target_id, tool_id]), changes));
        }
        Item::Stick => {
            if tool_type != &Item::Knife {
                return Err("Only a Knife can be used on a Stick!".to_owned());
            };
            make_axe(agent_id, tool_id, target_id)
        }
        Item::Wood => {
            if tool_type != &Item::Wood {
                return Err("Only a wood can be used on wood!".to_owned());
            };
            // get the agent's location in the world.
            let Ok((
                _,
                _,
                Location::World {
                    x: agent_x,
                    y: agent_y,
                },
                _,
            )) = query.get(agent_id)
            else {
                return Err("Actor not in the world!".to_owned());
            };
            let changes = vec![
                Changes::Despawn(target_id),
                Changes::Despawn(tool_id),
                Changes::SpawnLocationType {
                    location: Location::World {
                        x: *agent_x,
                        y: *agent_y,
                    },
                    tyep: Item::House,
                },
            ];
            return Ok((BTreeSet::from([target_id, tool_id]), changes));
        }
        Item::House => todo!(),
        Item::Tree => {
            if tool_type != &Item::Axe {
                return Err("Only an axe can be used on a Tree!".to_owned());
            };
            let changes = vec![
                Changes::Despawn(target_id),
                Changes::SpawnLocationType {
                    location: Location::Inventory(agent_id),
                    tyep: Item::Wood,
                },
            ];
            return Ok((BTreeSet::from([target_id]), changes));
        }
        Item::Veggie => {
            if tool_type != &Item::Knife {
                return Err("only a knife can be used ona Veggie!".to_owned());
            }
            // She used an knife so destroy the Veggie and make 3 Seeds
            let changes = vec![
                Changes::Despawn(target_id),
                Changes::SpawnLocationType {
                    location: Location::Inventory(agent_id),
                    tyep: Item::Seed,
                },
                Changes::SpawnLocationType {
                    location: Location::Inventory(agent_id),
                    tyep: Item::Seed,
                },
                Changes::SpawnLocationType {
                    location: Location::Inventory(agent_id),
                    tyep: Item::Seed,
                },
            ];
            return Ok((BTreeSet::from([target_id]), changes));
        }
    }
}

fn make_axe(
    agent_id: EntityId,
    knife_id: EntityId,
    stick_id: EntityId,
) -> Result<(BTreeSet<EntityId>, Vec<Changes>), String> {
    let changes = vec![
        Changes::Despawn(knife_id),
        Changes::Despawn(stick_id),
        Changes::SpawnLocationType {
            location: Location::Inventory(agent_id),
            tyep: Item::Axe,
        },
    ];
    return Ok((BTreeSet::from([knife_id, stick_id]), changes));
}
