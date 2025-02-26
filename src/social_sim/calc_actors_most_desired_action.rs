use std::collections::HashMap;

use qol::BiHashMap;

use crate::social_sim::{ActionId, ActorId, Desire};

use super::{get_most_desired_action, ActionDesire};

pub fn calc_actors_most_desired_action(
    actor_id: &ActorId,
    action_weights_hierarchy: &BiHashMap<ActorId, ActorId, HashMap<ActionId, Desire>>,
) -> Option<(ActionId, ActorId)> {
    let mut most_desired_action_maybe: Option<(ActionDesire, ActorId)> = None; //(ActionDesire, ActorId)> = HashMap::new();
    for (actee, weight_for_actions) in action_weights_hierarchy.get_inner().get(actor_id)? {
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
    }
    let most_desired_action = most_desired_action_maybe?;
    Some((most_desired_action.0.action_id, most_desired_action.1))
}
