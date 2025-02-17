use bevy::prelude::*;

#[derive(Debug, Clone, Event, PartialEq)]
pub struct ChangeConflict {
    pub hash: u64,
}
