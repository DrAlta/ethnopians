use bevy::prelude::*;

#[derive(Debug, Clone, Event, PartialEq)]
pub struct ActionConflict {
    pub hash: u64,
}
