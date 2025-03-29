use bevy::prelude::*;

use crate::sandbox::EntityId;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Result{
    Success,
    Failure,
}

#[derive(Event, Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct ActionResult {
    pub agent_id: EntityId,
    pub action_id: u64,
    pub result: Result
}
