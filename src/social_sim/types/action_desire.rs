use crate::social_sim::{ActionId, Desire};
#[derive(Debug, Clone)]
pub struct ActionDesire {
    pub action_id: ActionId,
    pub weight: Desire,
}
