type Number = f32;
type TagId = String;
pub fn is_happy(preferences: &Vec<(TagId, Number)>, game_tags: Vec<TagId>) -> Number {
    let mut and_score: Number = 1.0; // Start with 1.0 for multiplicative scoring.
    let mut or_numer: Number = 0.0; // Start with 0.0 for additive scoring.
    let mut or_demon: Number = 0.0; // Keep track of the total weights.

    for pref in preferences {
        let pref_weight = pref.1.min(1.0).max(0.0);
        let oriness = 1.0 - pref_weight;
        or_demon += oriness;
        if game_tags.contains(&pref.0) {
            and_score *= pref_weight; // Multiply for 'and'iness (product of weights).
            or_numer += oriness; // Add for 'or'iness (sum of weights).
        }
    }

    let or_score = (or_numer / or_demon).min(1.0);

    or_score.min(and_score)
}
