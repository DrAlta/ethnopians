use std::collections::HashMap;

use qol::BiHashMap;

use crate::{ActionId, ActorId, Desire};

use super::{get_most_desired_action, ActionDesire};

pub fn calc_actors_most_desired_action_for_each_actee(
    action_weights_hierarchy: &BiHashMap<ActorId, ActorId, HashMap<ActionId, Desire>>,
) -> HashMap<ActorId, (ActionDesire, ActorId)> {
    let mut most_desired_action: HashMap<ActorId, (ActionDesire, ActorId)> = HashMap::new();
    for ((actor, actee), weight_for_actions) in action_weights_hierarchy {
        let mut most_desired_action_maybe = most_desired_action.get(actor).cloned();

        match (
            &most_desired_action_maybe,
            get_most_desired_action(weight_for_actions),
        ) {
            (Some(most_desired_action), Some(new_action)) => {
                if new_action.weight > most_desired_action.0.weight {
                    most_desired_action_maybe = Some((new_action, actee.clone()));
                }
            }
            (None, Some(new_action)) => {
                most_desired_action_maybe = Some((new_action, actee.clone()));
            }
            (None, None) | (Some(_), None) => (),
        }

        let Some(action_most_desired) = most_desired_action_maybe else {
            continue;
        };

        most_desired_action.insert(actor.clone(), action_most_desired);
    }
    most_desired_action
}
