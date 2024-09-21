use std::collections::HashMap;

use qol::{logy, BiHashMap, PushOrInsert};

use crate::{ActionID, ActorID, Desire};

use super::h_plus;

/// this is wrong but might be useful. it collects the max desires of eveyone else to interacting with everyone that isn't interacting with someone 
pub fn calc_recieved_desire_total(
    action_weights_hierarchy: &BiHashMap<ActorID, ActorID, HashMap<ActionID, Desire>>,
) -> HashMap<ActorID, Desire>{
    let recieved_desires = action_weights_hierarchy
        .iter()
        .fold(
            HashMap::new(),
            |
                mut acc,
                (
                    (_initiator, responder), 
                    weigths
                )
            |  {
                /* this filters out colecting the receives desires for actors that have already chosen who to act with
                if init_interactions.iter().any(|&(a,b)| { a == responder || b == responder }) {
                    return acc;
                }
                */

                logy!("todo", "this is finding the highest Desire to interact with her, should we instead use thr HPlus of his desires to interact with her?");
                if let Some((_action_id, max_desire)) = weigths.iter().max_by(|&(_, a), &(_, b)| {
                    a.cmp(b)
                }) {
                    acc.push_or_insert(responder, max_desire);

                };
                acc
            }
        );
    recieved_desires.into_iter().map(
        |(actor_id, desires)|{
            (actor_id.clone(), h_plus(desires.into_iter().map(|x|x.clone())))
        }
    ).collect()

}
