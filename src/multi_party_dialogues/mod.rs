use std::collections::HashMap;

use crate::Number;
type Int = i32;

type CharId = u32;
type Probability = Number;

type I = HashMap<CharId, Int>;
type N = HashMap<CharId, Number>;

pub fn compute_turn_probabilities<'a,'b, 'c, 'd, 'e>(
    cue_strengths: &'a N, 
    social_statuses: &'b N, 
    attentions: &'c N,
    requests_made_by_char: &'d I, 
    uncontested_followups_by_char: &'e I,
)
-> HashMap<CharId, Probability> 
{
    if cue_strengths.into_iter().next().is_none() {
        return HashMap::new();
    }

    let effective_total_requests: Int = 
        requests_made_by_char.into_iter().map(|(_,x)| x).sum::<Int>()
        - uncontested_followups_by_char.into_iter().map(|(_,x)| x).sum::<Int>();

    let mut total_weight  = Number::ZERO;
    let mut character_weights = HashMap::new();

    
    for (id, cue_strength) in cue_strengths {
        let Some(status) = social_statuses.get(id) else {
            continue;
        };
        let Some(attention) = attentions.get(id) else {
            continue;
        };
        let Some(requests_made) = requests_made_by_char.get(id) else {
            continue;
        };
        let Some(uncontested_followups) = uncontested_followups_by_char.get(id) else {
            continue;
        };
        let hog_factor = Into::<Number>::into(requests_made - uncontested_followups) / Into::<Number>::into(effective_total_requests);
        let weight = (cue_strength * status * 0.7 + attention * cue_strength * 0.3) * (Number::ONE - hog_factor * 0.5);
        total_weight += weight;
        character_weights.insert(id, weight);
    }

    character_weights
     .into_iter()
     .map(|(id, weight)| (*id, weight / total_weight))
     .collect()
}