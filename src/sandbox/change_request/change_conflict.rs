use bevy::prelude::*;

#[derive(Event, Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct ChangeConflict {
    pub hash: u64,
}
