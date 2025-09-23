use std::collections::BTreeSet;
 use super::Preference;

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct Rule {
    // Stage 1: The activation set for the event context
    pub event_tags: BTreeSet<String>,
    // Stage 2: The activation set for the response context
    pub response_tags: BTreeSet<String>,
    pub preference: Preference,
}
