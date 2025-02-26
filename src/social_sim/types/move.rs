use crate::social_sim::{ActionId, ActorId};

#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Move {
    pub actor_id: ActorId,
    pub actee_id: ActorId,
    pub action_id: ActionId,
}

impl Move {
    pub fn new(actor_id: ActorId, actee_id: ActorId, action_id: ActionId) -> Self {
        Self {
            actor_id,
            actee_id,
            action_id,
        }
    }
}
