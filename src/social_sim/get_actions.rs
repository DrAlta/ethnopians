use std::collections::HashMap;

use qol::BiHashMap;

use crate::social_sim::{ActionId, ActorId, Desire, TimeIndex};

type IntentId = String;
type RelationId = String;
type RoleId = String;

type Action = ();
mod cozo {
    pub type DataValue = ();
}
type DbInstance = ();
type Value = ();
type Volition<'c> = &'c ();
type IntentOrActionId = ();

pub fn get_actions<'c>(
    _actions: &HashMap<ActionId, Action>,
    _intent_roots: &HashMap<IntentId, Vec<ActionId>>,
    _volitions: &BiHashMap<ActorId, ActorId, Vec<Volition<'c>>>,
    _now: TimeIndex,
    _db: &mut DbInstance,
    _bindings: &HashMap<String, cozo::DataValue>,
    (_defaults, _relations_roles, _durations): (
        &HashMap<RelationId, Value>,
        &HashMap<RelationId, Vec<RoleId>>,
        &HashMap<RelationId, TimeIndex>,
    ),
) -> Result<BiHashMap<ActorId, ActorId, HashMap<ActionId, HashMap<IntentOrActionId, Desire>>>, String>
{
    todo!()
}
