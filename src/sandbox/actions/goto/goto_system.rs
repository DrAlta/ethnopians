use bevy::prelude::*;
use qol::logy;

use crate::sandbox::{
    actions::{ActionResult, GotoRequest, Result},
    world::Movement,
    Collision, TravelCompleted,
};

#[derive(Component, Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct MovementRequest {
    pub prayer_id: u64,
}

pub fn goto_system(
    mut movement_query: Query<&mut Movement>,
    mut movement_request_query: Query<&mut MovementRequest>,
    mut goto_requests: EventReader<GotoRequest>,
    mut action_result: EventWriter<ActionResult>,

    mut collision_events: EventReader<Collision>,
    mut travel_completed_events: EventReader<TravelCompleted>,

    mut commands: Commands,
) {
    logy!("trace-goto", "entering goto_syetem");

    for Collision {
        agent_id,
        collider_id: _,
    } in collision_events.read()
    {
        let Ok(movement_request) = movement_request_query.get(*agent_id) else {
            continue;
        };
        action_result.send(ActionResult {
            agent_id: *agent_id,
            prayer_id: movement_request.prayer_id,
            result: Result::Failure,
        });
        commands.entity(*agent_id).remove::<MovementRequest>();
    }

    for TravelCompleted {
        entity_id: agent_id,
    } in travel_completed_events.read()
    {
        let Ok(movement_request) = movement_request_query.get(*agent_id) else {
            continue;
        };
        action_result.send(ActionResult {
            agent_id: *agent_id,
            prayer_id: movement_request.prayer_id,
            result: Result::Success,
        });
        commands.entity(*agent_id).remove::<MovementRequest>();
    }

    for GotoRequest {
        prayer_id,
        agent_id,
        movement,
    } in goto_requests.read()
    {
        let mut new = false;
        if let Ok(mut movement_request_component) = movement_request_query.get_mut(*agent_id) {
            if &movement_request_component.prayer_id == prayer_id {
                // we are ready doing this request
            } else {
                new = true;
                action_result.send(ActionResult {
                    agent_id: *agent_id,
                    prayer_id: *prayer_id,
                    result: Result::Failure,
                });
                *movement_request_component = MovementRequest {
                    prayer_id: *prayer_id,
                };
            }
        } else {
            commands.entity(*agent_id).insert(movement.clone());
        }
        if let Ok(mut movement_component) = movement_query.get_mut(*agent_id) {
            // re don't want to clober it is it an already existing action
            if new {
                *movement_component = movement.clone();
            }
        } else {
            commands.entity(*agent_id).insert(movement.clone());
        }
    }
    logy!("trace-goto", "exiting goto_syetem");
}
