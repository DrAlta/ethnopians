use bevy::prelude::*;
use qol::logy;

use crate::sandbox::{
    actions::{PosibleActionsRequest, PosibleActionsResponce, use_object::{Command, use_object, UseRequest}},
    ActionId,
    world::{Energy, Hp, Size, Type}, Location,
};


pub fn use_object_system(
    mut query: Query<(Entity, &Type, &Location, Option<&Size>, Option<&mut Energy>, Option<&mut Hp>)>,
    mut use_requests: EventReader<UseRequest>,
    mut posible_actions_requests: EventReader<PosibleActionsRequest>,
    mut posible_actions_responce: EventWriter<PosibleActionsResponce>,
    mut commands: Commands,
) {
    logy!("trace-use-object", "entering use_object_syetem");
    for UseRequest { agent_id, target_id } in use_requests.read()
    {
        match use_object(&query, *agent_id, *target_id) {
            Ok(command) => {
                logy!("trace-use-object", "{agent_id:?} used {target_id:?}");
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
            Err(_err) => {
                logy!("trace-use-object", "{agent_id:?} failed toto use {target_id:?} because:{_err}");
            }
        }
        {}
    }
    for PosibleActionsRequest {
        agent_id,
        target_id,
    } in posible_actions_requests.read()
    {
        match use_object(&query, *agent_id, *target_id) {
            Ok(_) => {
                logy!("trace-use-object", "sending respone that {agent_id:?} is able to use {target_id:?}");
                posible_actions_responce.send(PosibleActionsResponce {
                    agent_id: *agent_id,
                    target_id: *target_id,
                    action_id: ActionId::UseObject,
                });
            }
            Err(_err) => {
                logy!("trace-use-object", "{agent_id:?} is unable to use {target_id:?} because:{_err}");
            }
        }
        {}
    }
    logy!("trace-use-object", "exiting use_object_syetem");
}
