use crate::{ActionID, Desire};
#[derive(Debug, Clone)]
pub struct ActionDesire {
    pub action_id: ActionID,
    pub weight: Desire
}
