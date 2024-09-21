use std::collections::{HashMap, HashSet};

use qol::{logy, BiHashMap};
use crate::{ActionID, ActorID, Desire, TimeIndex};

//start placeholder
mod get_actions;
pub use get_actions::get_actions;
//end placeholder

mod action_desire;
pub use action_desire::ActionDesire;
mod calc_actors_most_desired_action_for_each_acte;
pub use calc_actors_most_desired_action_for_each_acte::calc_actors_most_desired_action_for_each_actee;
mod calc_recieved_desire_total;
pub use calc_recieved_desire_total::calc_recieved_desire_total;
mod h_plus;
pub use h_plus::h_plus;


type IntentID = String;
type RelationID = String;
type RoleID = String;

type ResponderId = ActorID;
type InitiatorId = ActorID;
type Action = ();
mod cozo{
   pub type DataValue = ();
}
type DbInstance =();
type Value = ();
type Volition<'c> = &'c ();
type IntentOrActionID = ();


fn total_action_weights(_action_heirarchy:BiHashMap::<ActorID, ActorID, HashMap<ActionID, HashMap<IntentOrActionID, Desire>>>) ->
    BiHashMap::<ActorID, ActorID, HashMap::<ActionID, Desire>>
{
    todo!()
}

////
/*
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
){
    // 1 the set $PrevInteractions is empty
    let prev_interactions = HashSet::new();

    // 2a Calculate the desire to do every action to every person, 
    let Ok(action_hierarchy) = get_actions( 
        actions, 
        intent_roots, 
        volitions, 
        now, 
        db, 
        bindings, 
        (
            defaults, 
            relations_roles, 
            durations
        )
    ) else {
        logy!("error", "got empty action hierarchy");
        return;
    };
    let action_weights_hierarchy = total_action_weights(action_hierarchy);

    // 2b if they both want to interact with each other they do add the char of the pair with highest desire to the set $InitInteractions
    let actors_most_desired_action_for_each_actee = calc_actors_most_desired_action_for_each_actee(&action_weights_hierarchy);

    let mut init_interactions = HashSet::new();

    for (initiator, (_action_most_desired, responder)) in &actors_most_desired_action_for_each_actee{
        let _responders_desired_action = actors_most_desired_action_for_each_actee.get(responder);

        let (lowest_char_id, highest_char_id) = if responder < initiator {
            (responder, initiator)
        } else {
            (initiator, responder)
        };
        init_interactions.insert((lowest_char_id, highest_char_id));
    }

    // 3 if $InitInteractions is not a subset of $PrevInteractions have everyone in $InitInteractions interact and go to step 4. If $InitInteractions is a subset of $PrevInteractions the scene ends.
    if init_interactions.is_subset(&prev_interactions) {
        return
    };

    // 4 Calculate <X>FD<Y>= Hplus of char X's desires to interact with char Y that isn't already interacting.(their desire to interact with Y)
    let full_desire: BiHashMap<InitiatorId, ResponderId, Desire> = action_weights_hierarchy.iter().filter_map(
        |((initiator, responder), weight_for_actions)|{

            let (lowest_char_id, highest_char_id) = if responder < initiator {
                (responder, initiator)
            } else {
                (initiator, responder)
            };
            if init_interactions.contains(&(lowest_char_id, highest_char_id)) {
                return None;
            }
            let h_plused = h_plus::<Desire,_,_>(
                weight_for_actions.iter().map(
                    |(_, weight)| weight.clone()
                )
            );

            Some(((initiator, responder), h_plused))
        }
    ).collect();
    // 5 foreach char X that isn't interacting, calculate <X>RDtotal = Hplus of char X's desires to interact with everyone that wants to interact with them.(their desire to interact with someone)

    // lets start out be finding who everone wants to interact with
    let who_they_want_to_interact_with: BiHashMap<ActorID, ActorID, Desire> = action_weights_hierarchy
        .get_inner()
        .iter()
        .filter_map(
            |
                (
                    initiator, 
                    desires_to_interact_with_people
                )
            | {
                let (most_desired_to_talk_to, desire_to_talk_to_most_desired) = desires_to_interact_with_people
                    .iter()
                    .filter_map( |(responder, desires)|{
                        let x =desires
                            .iter()
                            .max_by(
                                |
                                &(_, a), 
                                &(_, b)
                            | {
                                a.cmp(b)
                            });
                        let (
                            _action_id, 
                            max_desire
                        ) = x ?;

                        Some((responder, max_desire))
                    })
                    // now we have an iter over their max desires to interacter with each character
                    .max_by(
                        |
                        &(_actor_a_id, max_desire_to_interact_with_a), 
                        &(_actor_b_id, max_desire_to_interact_with_b)
                    | {
                        max_desire_to_interact_with_a.cmp(max_desire_to_interact_with_b)
                    })?;
                
                let x = ((initiator, most_desired_to_talk_to), desire_to_talk_to_most_desired.clone());
                Some(x)
            }
        ).collect();
/*
    let returned_desires = action_weights_hierarchy
        .iter()
        .filter_map(
            |
                (
                    (initiator, responder), 
                    weigths
                )
            |  {
                if init_interactions.iter().any(|&(a,b)| { a == responder || b == responder }) {
                    return None;
                }

                let responders = action_weights_hierarchy
                    .get_inner()
                    .get(responder)?
                    .iter()
                    .
                    .get(initiator)?
                    .iter().max_by(|&(_, desire_to_interact_with_initiator), &(_, b)| {
                        a.cmp(b)
                    });

                let Some((_action_id, max_desire)) = weigths.iter().max_by(|&(_, a), &(_, b)| {
                    a.cmp(b)
                })?;
                max_desire
            }
        );

*/
    // 6 Char A, that isn't already interacting, with the highest <nowiki><A>RDtotal</nowiki> interacts with a the char B that wants to interact with them, that has the highest <nowiki><A>FD<B></nowiki>. if ether is tied choose the one with the <nowiki><B>FD<A></nowiki> is highest

}




////////////////////////
fn get_most_desired_action(weight_for_actions: &HashMap<ActionID, Desire>) -> Option<ActionDesire> {
    let _ = weight_for_actions;
    todo!()
}


