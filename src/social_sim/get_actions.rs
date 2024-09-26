use std::collections::HashMap;

use qol::BiHashMap;

use crate::{ActionID, ActorID, Desire, TimeIndex};

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

pub fn get_actions<'c>(
    _actions: &HashMap<ActionID, Action>,
    _intent_roots: &HashMap<IntentID, Vec<ActionID>>,
    _volitions: &BiHashMap<ActorID, ActorID, Vec<Volition<'c>>>,
    _now: TimeIndex,
    _db: &mut DbInstance,
    _bindings: &HashMap<String, cozo::DataValue>,
    (_defaults, _relations_roles, _durations): (
        &HashMap<RelationID, Value>,
        &HashMap<RelationID, Vec<RoleID>>,
        &HashMap<RelationID, TimeIndex>,
    ),
) -> Result<BiHashMap<ActorID, ActorID, HashMap<ActionID, HashMap<IntentOrActionID, Desire>>>, String>
{
    todo!()
}
