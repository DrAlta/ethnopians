use std::collections::BTreeSet;

use bevy::prelude::*;

use crate::sandbox::{EntityId, change_request::Changes};


#[derive(Debug, Event, PartialEq)]
pub struct ChangeRequest {
    pub hash: u64,
    pub contentious_entities: BTreeSet<EntityId>,
    pub changes: Vec<Changes>,
}