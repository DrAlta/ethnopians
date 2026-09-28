pub fn effective_fatigue(true_fat: f64, favorite: Option<f64>) -> f64 {
    // Rule 1: Non-favorite items have zero transformation
    let Some(fav) = favorite else {
        return true_fat;
    };

    // Rule 2: Favorite items have complex threshold behavior
    if true_fat > fav {
          fav *(1.0 - (
            0.5_f64.powf(
               ((true_fat-fav) / fav) 
            ))
        ) + fav
    } else {
        // BEFORE THRESHOLD: The higher the fav tolerance, the lower the effective boredom is!
        let t = if fav <= 0.0 { 1.0 } else { true_fat / fav };
        true_fat * t
    }
}

