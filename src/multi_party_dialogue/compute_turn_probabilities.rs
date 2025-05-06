// Import the standard library's HashMap collection
use std::collections::HashMap;

// Import the Number type from the crate
use crate::Number;

// Define a type alias for Int as i32
type Int = i32;

// Define type aliases for CharId and Probability
type CharId = u32;
type Probability = Number;

// Define type aliases for HashMaps with CharId as the key and Int or Number as the value
type I = HashMap<CharId, Int>;
type N = HashMap<CharId, Number>;

// Define a function to compute turn probabilities for characters
// This function takes in several HashMaps as input:
// - cue_strengths: a map of character IDs to their cue strengths
// - social_statuses: a map of character IDs to their social statuses
// - attentions: a map of character IDs to the attention they're receiving
// - requests_made_by_char: a map of character IDs to the number of requests they've made
// - uncontested_followups_by_char: a map of character IDs to the number of uncontested follow-ups they've made
pub fn compute_turn_probabilities<'a, 'b, 'c, 'd, 'e>(
    cue_strengths: &'a N,
    social_statuses: &'b N,
    attentions: &'c N,
    requests_made_by_char: &'d I,
    uncontested_followups_by_char: &'e I,
) -> HashMap<CharId, Probability> {
    // Compute the effective total number of requests
    // This is done by summing up the total number of requests made by all characters
    // and subtracting the total number of uncontested follow-ups
    // The reason for subtracting the uncontested follow-ups is to avoid double-counting
    // requests that were made as part of an uncontested sequence
    let effective_total_requests: Int = requests_made_by_char
        .into_iter()
        .map(|(_, x)| x)
        .sum::<Int>()
        - uncontested_followups_by_char
            .into_iter()
            .map(|(_, x)| x)
            .sum::<Int>();

    // Initialize the total weight and character weights HashMap
    // The total weight will be used to normalize the character weights
    let mut total_tickets = Number::ZERO;
    let mut character_tickets = HashMap::new();

    // Iterate over the cue strengths map
    // For each character, compute their weight based on their cue strength, social status, and attention
    // The weight is computed as a weighted sum of the cue strength and attention
    // The weights are 0.7 and 0.3, respectively, which means that the cue strength is given more importance
    for (id, cue_strength) in cue_strengths {
        // Get the social status and attention for the current character
        // If the social status or attention is not found, skip the current character
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

        // Compute the hog factor for the current character
        // The hog factor is a measure of how much the character is dominating the conversation
        // It's computed as the ratio of the character's requests minus their uncontested follow-ups
        // to the effective total number of requests
        let hog_factor = Into::<Number>::into(requests_made - uncontested_followups)
            / Into::<Number>::into(effective_total_requests);

        // Compute the weight for the current character
        // The weight is computed as the sum of the cue strength and attention
        // minus a penalty term based on the hog factor
        // The penalty term is used to discourage characters from dominating the conversation
        // we keep the range of the penalty to 0.5 - 1.0 so that their probabilty doesn't 
        // go to 0.0 when they are the only one talking
        let tickets = (cue_strength * status + attention * cue_strength)
            * (Number::ONE - hog_factor * Number::HALF);

        // Add the weight to the total weight and insert it into the character weights map
        total_tickets += tickets;
        character_tickets.insert(id, tickets);
    }

    // Normalize the character weights by dividing them by the total weight
    // This ensures that the weights sum up to 1
    character_tickets
        .into_iter()
        .map(|(id, tickets)| (*id, tickets / total_tickets))
        .collect()
}
