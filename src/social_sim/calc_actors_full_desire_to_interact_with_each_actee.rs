use std::collections::HashMap;

use qol::BiHashMap;

use crate::{social_sim::{ActionId, ActorId, Desire}, util::h_plus};

pub fn calc_actors_full_desire_to_interact_with_each_actee(
    action_weights_hierarchy: &BiHashMap<ActorId, ActorId, HashMap<ActionId, Desire>>,
) -> BiHashMap<ActorId, ActorId, Desire> {
    action_weights_hierarchy
        .iter()
        .filter_map(|((initiator, responder), weight_for_actions)| {
            /* this filtered out already interacting actors

            let (lowest_char_id, highest_char_id) = if responder < initiator {
                (responder, initiator)
            } else {
                (initiator, responder)
            };
            if init_interactions.contains(&(lowest_char_id, highest_char_id)) {
                return None;
            }
            */
            let h_plused =
                h_plus::<Desire, _, _>(weight_for_actions.iter().map(|(_, weight)| weight.clone()));

            Some(((initiator, responder), h_plused))
        })
        .collect()
}
