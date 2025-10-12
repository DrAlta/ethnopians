use super::{is_more_specific, Character, Event, Preference, Response};

type Approvals = i32;
type Disapprovals = i32;

pub fn sum_preferences(
    character: &Character,
    event: &Event,
    response: &Response,
) -> (Approvals, Disapprovals) {
    let mut rule_candidates = Vec::new();

    // Stage 1: Find all rules whose event part is activated
    for rule in &character.rules {
        if event.tags.is_superset(&rule.event_tags) {
            rule_candidates.push(rule);
        }
    }

    // Step 2: find the subset with the highest specificity
    {
        let mut to_remove = Vec::new();
        for this_candidate_idx in 0..rule_candidates.len() {
            let this_candidate = &rule_candidates[this_candidate_idx].event_tags;
            for other_candidate_idx in this_candidate_idx + 1..rule_candidates.len() {
                let other_candidate = &rule_candidates[other_candidate_idx].event_tags;
                if is_more_specific(other_candidate, this_candidate) {
                    //other_candidate.is_superset(this_candidate) {
                    to_remove.push(this_candidate_idx);
                }
            }
        }
        while let Some(idx) = to_remove.pop() {
            rule_candidates.remove(idx);
        }
    }

    // step 3 gather the response test
    let mut response_candidates = Vec::new();
    for rule in rule_candidates {
        if response.tags.is_superset(&rule.response_tags) {
            response_candidates.push(rule);
        }
    }

    // Step 4: find the subset with the highest specificity
    {
        let mut to_remove = Vec::new();
        for this_candidate_idx in 0..response_candidates.len() {
            let this_candidate = &response_candidates[this_candidate_idx].response_tags;
            for other_candidate_idx in this_candidate_idx + 1..response_candidates.len() {
                let other_candidate = &response_candidates[other_candidate_idx].response_tags;
                if is_more_specific(other_candidate, this_candidate) {
                    //other_candidate.is_superset(this_candidate) {
                    to_remove.push(this_candidate_idx);
                }
            }
        }
        while let Some(idx) = to_remove.pop() {
            response_candidates.remove(idx);
        }
    }

    let mut approvals = 0;
    let mut disapprovals = 0;
    for rule in response_candidates {
        match &rule.preference {
            Preference::Approve => approvals += 1,
            Preference::Disapprove => disapprovals += 1,
        }
    }
    (approvals, disapprovals)
}
pub fn determine_preference(
    character: &Character,
    event: &Event,
    response: &Response,
) -> Option<Preference> {
    let (approvals, disapprovals) = sum_preferences(character, event, response);

    match approvals.cmp(&disapprovals) {
        std::cmp::Ordering::Less => Some(Preference::Disapprove),
        std::cmp::Ordering::Equal => None,
        std::cmp::Ordering::Greater => Some(Preference::Approve),
    }
}
