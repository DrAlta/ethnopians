mod fatigue;
mod fav;
pub fn compute_meal_boredom(ingredients: &[(fatigue::NoveltyScheduler, Option<f64>)], days_passed: f64) -> f64 {
    if ingredients.is_empty() { return 0.0; }

    // 1. Map to EFFECTIVE fatigue scores (applying the favorite bias)
    let effective_fatigues: Vec<f64> = ingredients.iter().map(|(ing, fav)| {
        fav::effective_fatigue(ing.get_current_fatigue(days_passed), *fav)
    }).collect();

    // 2. Find baseline effective average to split groups
    let initial_sum: f64 = effective_fatigues.iter().sum();
    let initial_avg = initial_sum / (effective_fatigues.len() as f64);

    let mut lowest_value = effective_fatigues.iter().min().unwrap_or(0.0); 
    let mut high_group = Vec::new();
    let mut low_group = Vec::new();

    for fatigue in effective_fatigues {
        if fatigue < lowest_value {
            lowest_value = fatigue;
        }
        if fatigue > initial_avg {
            high_group.push(fatigue);
        } else {
            low_group.push(fatigue);
        }
    }

    // 3. Sub-group averages
    let high_avg = if !high_group.is_empty() {
        high_group.iter().sum::<f64>() / (high_group.len() as f64)
    } else {
        initial_avg
    };

    let low_avg = if !low_group.is_empty() {
        low_group.iter().sum::<f64>() / (low_group.len() as f64)
    } else {
        initial_avg
    };

    // 4. Final 3-Value Average
    ((high_avg + low_avg + lowest_value) / 3.0).clamp(0.0, 1.0)
}
