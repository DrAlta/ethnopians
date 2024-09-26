use std::collections::HashMap;

use crate::{ActionID, ActorID, Desire, TimeIndex};
use qol::{logy, BiHashMap};

//start placeholder
mod get_actions;
pub use get_actions::get_actions;
//end placeholder

mod action_desire;
pub use action_desire::ActionDesire;

mod actors_most_desired_person_to_interact_with;
pub use actors_most_desired_person_to_interact_with::actors_most_desired_person_to_interact_with;

mod calc_actors_full_desire_to_interact_with_each_actee;
pub use calc_actors_full_desire_to_interact_with_each_actee::calc_actors_full_desire_to_interact_with_each_actee;

mod calc_actors_most_desired_action_for_each_actee;
pub use calc_actors_most_desired_action_for_each_actee::calc_actors_most_desired_action_for_each_actee;

mod calc_actors_most_desired_action_with_actee;
pub use calc_actors_most_desired_action_with_actee::calc_actors_most_desired_action_with_actee;

mod calc_actors_most_desired_action;
pub use calc_actors_most_desired_action::calc_actors_most_desired_action;

mod calc_recieved_desire_total;
pub use calc_recieved_desire_total::calc_recieved_desire_total;

mod calc_h_plus_of_desires_towards_actors;
pub use calc_h_plus_of_desires_towards_actors::calc_h_plus_of_desires_towards_actors;

mod calc_h_plus_of_full_desires_towards_actors;
pub use calc_h_plus_of_full_desires_towards_actors::calc_h_plus_of_full_desires_towards_actors;

mod h_plus;
pub use h_plus::h_plus;
mod r#move;
pub use r#move::Move;

type IntentID = String;
type RelationID = String;
type RoleID = String;

type Action = ();
mod cozo {
    pub type DataValue = ();
}
type DbInstance = ();
type Value = ();
type Volition<'c> = &'c ();
type IntentOrActionID = ();

fn total_action_weights(
    _action_heirarchy: BiHashMap<
        ActorID,
        ActorID,
        HashMap<ActionID, HashMap<IntentOrActionID, Desire>>,
    >,
) -> BiHashMap<ActorID, ActorID, HashMap<ActionID, Desire>> {
    todo!()
}

////
/*
It's not clear what I emnt by 'want to interact with' I'm taking it as meaning as they want to interact with the person that has the highist HPlus of their desires to interact with them


1 the set $PrevInteractions is empty
2 Calculate the desire to do every action to every person, if they both want to interact with each other they do add the char of the pair with highest desire to the set $InitInteractions
3 if $InitInteractions is not a subset of $PrevInteractions have every one in $InitInteractions interact and go to step 4. If $InitInteractions is a subset of $PrevInteractions the scene ends.
4 Calculate <nowiki><X>FD<Y></nowiki>= Hplus of char X's desires to interact with char Y that isn't already interacting.(their desire to interact with Y)
5 forech char X that isn't interacting, calculate <nowiki><X>RDtotal</nowiki>= Hplus of char X's desires to interact with everyone that wants to interact with them.(their desire to interact with someone)
6 Char A, that isn't already interacting, with the highest <nowiki><A>RDtotal</nowiki> interacts with a the char B that wants to interact with them, that has the highest <nowiki><A>FD<B></nowiki>. if ether is tied choose the one with the <nowiki><B>FD<A></nowiki> is highest
7 if someone wants to interact with someone that isn't interacting go to step 6
8 go to step 2
*/
pub fn social_sim<'c>(
    actions: &HashMap<ActionID, Action>,
    intent_roots: &HashMap<IntentID, Vec<ActionID>>,
    volitions: &BiHashMap<ActorID, ActorID, Vec<Volition<'c>>>,
    now: TimeIndex,
    db: &mut DbInstance,
    bindings: &HashMap<String, cozo::DataValue>,
    (defaults, relations_roles, durations): (
        &HashMap<RelationID, Value>,
        &HashMap<RelationID, Vec<RoleID>>,
        &HashMap<RelationID, TimeIndex>,
    ),
) {
    // 1 the set $PrevInteractions is empty
    let prev_interactions = HashMap::<ActorID, Option<Move>>::new();

    // 2a Calculate the desire to do every action to every person,
    let Ok(action_hierarchy) = get_actions(
        actions,
        intent_roots,
        volitions,
        now,
        db,
        bindings,
        (defaults, relations_roles, durations),
    ) else {
        logy!("error", "got empty action hierarchy");
        return;
    };
    let action_weights_hierarchy = total_action_weights(action_hierarchy);

    // 2b if they both want to interact with each other they do add the char of the pair with highest desire to the set $InitInteractions
    let mut init_interactions = HashMap::new();

    let full_desire: BiHashMap<ActorID, ActorID, Desire> =
        calc_actors_full_desire_to_interact_with_each_actee(&action_weights_hierarchy);

    let actors_most_desired_person_to_interact_with =
        actors_most_desired_person_to_interact_with(&full_desire);

    // 2c now we see is they want to interact with each other.
    for (actor_id, actee_id) in &actors_most_desired_person_to_interact_with {
        if let Some(thing) = actors_most_desired_person_to_interact_with.get(actee_id) {
            if actor_id == thing {
                init_interactions.insert(actor_id.clone(), actee_id.clone());
            }
        }
    }

    // 3 if $InitInteractions is not a subset of $PrevInteractions have everyone in $InitInteractions interact and go to step 4. If $InitInteractions is a subset of $PrevInteractions the scene ends.
    // 3a is any item on init_interactions is not in prev_interactions return early
    if init_interactions.iter().any(|(actor_id, actee_id)| {
        // if actor was the that made initatde the move check is the actee was the same
        if let Some(Some(Move {
            actor_id: _,
            actee_id: prev_actee,
            action_id: _,
        })) = prev_interactions.get(actor_id)
        {
            return actee_id != prev_actee;
        }
        // the actor wasn't the initator so check if the actee initated an action with the actor
        let Some(Some(Move {
            actor_id: prev_actor_id,
            actee_id: _,
            action_id: _,
        })) = prev_interactions.get(actee_id)
        else {
            return true;
        };
        return actor_id != prev_actor_id;
    }) {
        return;
    };
    // 3b init_interaction was a subset on prev_interaction so now have the the inital actor interact
    let mut interactions = HashMap::<ActorID, Option<Move>>::new();
    for (actor_id, _) in &init_interactions {
        let Some(actee_id) = actors_most_desired_person_to_interact_with.get(actor_id) else {
            continue;
        };
        match (
            calc_actors_most_desired_action_with_actee(
                actor_id,
                actee_id,
                &action_weights_hierarchy,
            ),
            calc_actors_most_desired_action_with_actee(
                actee_id,
                actor_id,
                &action_weights_hierarchy,
            ),
        ) {
            (None, None) => continue,
            (
                None,
                Some(ActionDesire {
                    action_id: actees_desired_action_id,
                    weight: _actees_desire_for_actor,
                }),
            ) => {
                interactions.insert(
                    actee_id.clone(),
                    Some(Move::new(
                        actee_id.clone(),
                        actor_id.clone(),
                        actees_desired_action_id,
                    )),
                );
                interactions.insert(actor_id.clone(), None);
            }
            (
                Some(ActionDesire {
                    action_id: actors_desired_action_id,
                    weight: _actors_desire_for_actee,
                }),
                None,
            ) => {
                interactions.insert(
                    actor_id.clone(),
                    Some(Move::new(
                        actor_id.clone(),
                        actee_id.clone(),
                        actors_desired_action_id,
                    )),
                );
                interactions.insert(actee_id.clone(), None);
            }
            (
                Some(ActionDesire {
                    action_id: actors_desired_action_id,
                    weight: actors_desire_for_actee,
                }),
                Some(ActionDesire {
                    action_id: actees_desired_action_id,
                    weight: actees_desire_for_actor,
                }),
            ) => {
                if actors_desire_for_actee >= actees_desire_for_actor {
                    interactions.insert(
                        actor_id.clone(),
                        Some(Move::new(
                            actor_id.clone(),
                            actee_id.clone(),
                            actors_desired_action_id,
                        )),
                    );
                    interactions.insert(actee_id.clone(), None);
                } else {
                    interactions.insert(
                        actee_id.clone(),
                        Some(Move::new(
                            actee_id.clone(),
                            actor_id.clone(),
                            actees_desired_action_id,
                        )),
                    );
                    interactions.insert(actor_id.clone(), None);
                }
            }
        };
    }

    // 4 Calculate <X>FD<Y>= Hplus of char X's desires to interact with char Y that isn't already interacting.(their desire to interact with Y)

    // we caled the FD for all char earlier

    // 5 foreach char X that isn't interacting, calculate <X>RDtotal = Hplus of char X's desires to interact with everyone that wants to interact with them.(their desire to interact with someone)

    //let returned_desires =

    // 6 Char A, that isn't already interacting, with the highest <nowiki><A>RDtotal</nowiki> interacts with a the char B that wants to interact with them, that has the highest <nowiki><A>FD<B></nowiki>. if ether is tied choose the one with the <nowiki><B>FD<A></nowiki> is highest
}

////////////////////////
fn get_most_desired_action(weight_for_actions: &HashMap<ActionID, Desire>) -> Option<ActionDesire> {
    let _ = weight_for_actions;
    todo!()
}
