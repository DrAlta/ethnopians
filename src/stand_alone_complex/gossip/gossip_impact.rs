use crate::Number;

use super::OpinionChange;

/// Represents the potential changes to a character's beliefs resulting from hearing gossip.
pub struct GossipImpact {
    /// Change in trust towards the gossiper.
    pub trust_change: Number,

    /// Change in affection towards the victim of the gossip.
    pub affection_change_towards_victim: Number,

    /// Changes in perceptions about how the subject feels about the victim (for third-party gossip).
    pub perceptions_update: Option<OpinionChange>,
}
