use std::collections::HashMap;

use qol::BiHashMap;

use crate::social_sim::{ActionId, ActorId, Desire};

use super::{get_most_desired_action, ActionDesire};

pub fn calc_actors_most_desired_action_with_actee(
    actor_id: &ActorId,
    actee_id: &ActorId,
    action_weights_hierarchy: &BiHashMap<ActorId, ActorId, HashMap<ActionId, Desire>>,
) -> Option<ActionDesire> {
    let actors_weight_for_actions = action_weights_hierarchy.get_inner().get(actor_id)?;
    let weight_for_actions_with_actee = actors_weight_for_actions.get(actee_id)?;
    get_most_desired_action(weight_for_actions_with_actee)
}
