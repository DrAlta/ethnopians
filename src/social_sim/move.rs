use crate::{ActionID, ActorID};

#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Move{
    pub actor_id: ActorID,
    pub actee_id: ActorID,
    pub action_id: ActionID,
}

impl Move {
    pub fn new(
        actor_id: ActorID,
        actee_id: ActorID,
        action_id: ActionID,
    ) -> Self {
        Self { actor_id, actee_id, action_id }
    }
}