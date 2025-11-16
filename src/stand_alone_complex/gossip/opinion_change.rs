use crate::Number;

use super::CharId;

/// Represents a change in perception about how one character feels about another.
pub struct OpinionChange {
    pub subject_id: CharId,
    pub victim_id: CharId,
    pub affection_change: Number,
}
