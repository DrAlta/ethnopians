use std::collections::HashMap;

use qol::{BiHashMap, PushOrInsert};

use crate::{ActorID, Desire};

use super::h_plus;

pub fn calc_h_plus_of_full_desires_towards_actors(
    full_desires: &BiHashMap<ActorID, ActorID, Desire>,
) -> HashMap<ActorID, Desire> {
    let mut acc = HashMap::new();
    for ((_actor, actee), desire) in full_desires {
        acc.push_or_insert(actee, desire);
    }
    acc.into_iter()
        .map(|(actor_id, desires)| {
            (
                actor_id.clone(),
                h_plus(desires.into_iter().map(|x| x.clone())),
            )
        })
        .collect()
}
