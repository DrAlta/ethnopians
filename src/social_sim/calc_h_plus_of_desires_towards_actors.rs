use std::collections::HashMap;

use qol::{BiHashMap, PushOrInsert};

use crate::{ActionID, ActorID, Desire};

use super::h_plus;

pub fn calc_h_plus_of_desires_towards_actors(
    action_weights_hierarchy: &BiHashMap<ActorID, ActorID, HashMap<ActionID, Desire>>
) -> HashMap<ActorID, Desire>{
    let mut acc = HashMap::new();
    for ((_actor, actee), weights) in action_weights_hierarchy {
        let mut weights2: Vec<Desire> = weights.iter().map(|(_action_id, desire) | desire.clone()).collect();
        acc.append_or_insert(actee, &mut weights2);
    }
    acc
        .into_iter()
        .map(
            |(actor_id, desires)| 
                (
                    actor_id.clone(), 
                    h_plus(desires.into_iter())
                )
        ).collect()
}