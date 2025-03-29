use crate::social_sim::{ActionId, Desire};
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct ActionDesire {
    pub action_id: ActionId,
    pub weight: Desire,
}
