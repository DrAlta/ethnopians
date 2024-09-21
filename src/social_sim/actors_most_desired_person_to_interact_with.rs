use std::collections::HashMap;

use qol::BiHashMap;

use crate::{ActorID, Desire};

pub fn actors_most_desired_person_to_interact_with(
    full_desires: &BiHashMap<ActorID, ActorID, Desire>
) -> HashMap<ActorID, ActorID> {
    full_desires.get_inner().iter().map(|(actor, desires)| {
        let (desided_partner, _desire) = desires
            .iter()
            .max_by(
                |&(_actee_id_a, desire_a), &(_actee_id_b, desire_b)| desire_a.cmp(desire_b)
            ).expect("if were are itering over the desires then there should by atleat one so there should bea max");
        (actor.clone(), desided_partner.clone())
    }).collect()
}