use std::hash::{DefaultHasher, Hash, Hasher};

use bevy::prelude::*;
use qol::logy;

use crate::{
    sandbox::{
        actions::{
            use_on::{use_on, UseOnRequest},
            //PosibleActionsRequest, PosibleActionsResponce,
        },
        change_request::ChangeRequest,
        world::{Size, Type},
        Location,
    },
    //types::ActionId,
};

pub fn use_on_system(
    query: Query<(Entity, &Type, &Location, Option<&Size>)>,
    mut use_on_requests: EventReader<UseOnRequest>,
    //mut posible_actions_requests: EventReader<PosibleActionsRequest>,
    //mut posible_actions_responce: EventWriter<PosibleActionsResponce>,
    mut commands: Commands,
) {
    logy!("trace-use-on", "entering use_on_syetem");
    let salt = 0;
    for UseOnRequest {
        agent_id,
        tool_id,
        target_id,
    } in use_on_requests.read()
    {
        let mut s = DefaultHasher::new();
        salt.hash(&mut s);
        "UseOn".hash(&mut s);
        agent_id.hash(&mut s);
        target_id.hash(&mut s);
        let hash = s.finish();

        match use_on(&query, *agent_id, *tool_id, *target_id) {
            Ok((contentious_entities, changes)) => {
                logy!("trace-use-object", "{agent_id:?} used {target_id:?}");
                commands.send_event(ChangeRequest {
                    hash,
                    contentious_entities,
                    changes,
                });
            }
            Err(_err) => {
                logy!(
                    "trace-use-on",
                    "{agent_id:?} failed toto use {target_id:?} because:{_err}"
                );
            }
        }
        {}
    }
    /*
    for PosibleActionsRequest {
        agent_id,
        target_id,
    } in posible_actions_requests.read()
    {
        match use_on(&query, *agent_id, *tool_id, *target_id) {
            Ok(_) => {
                logy!(
                    "trace-use-object",
                    "sending respone that {agent_id:?} is able to use {target_id:?}"
                );
                posible_actions_responce.send(PosibleActionsResponce {
                    agent_id: *agent_id,
                    target_id: *target_id,
                    action_id: ActionId::USE_OBJECT,
                });
            }
            Err(_err) => {
                logy!(
                    "trace-use-object",
                    "{agent_id:?} is unable to use {target_id:?} because:{_err}"
                );
            }
        }
        {}
    }*/
    logy!("trace-use-on", "exiting use_on_syetem");
}
