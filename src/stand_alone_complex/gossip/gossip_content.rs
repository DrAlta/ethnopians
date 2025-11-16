use crate::Number;
use super::CharId;

/// Represents the content of a gossip shared by a gossiper.
pub struct GossipContent {
    /// The ID of the subject of the gossip.
    /// If it's direct gossip, `subject_id` is the same as the `gossiper`'s ID.
    pub subject_id: CharId,

    /// The ID of the victim (the character being talked about).
    pub victim_id: CharId,

    /// The affection value expressed in the gossip.
    /// Range: -1.0 (strong negative sentiment) to 1.0 (strong positive sentiment).
    pub affection: Number,
}