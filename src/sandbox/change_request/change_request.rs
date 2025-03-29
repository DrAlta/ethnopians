use std::collections::BTreeSet;

use bevy::prelude::*;

use crate::sandbox::{change_request::Changes, EntityId};

#[derive(Event, Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct ChangeRequest {
    pub hash: u64,
    pub contentious_entities: BTreeSet<EntityId>,
    pub changes: Vec<Changes>,
}
