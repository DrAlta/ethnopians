pub fn staple_fatigue(true_fatigue: f64, staple: f64) -> f64 {
    if true_fatigue < 0.0 {
        return true_fatigue;
    }

    if staple == 0.0 {
        return (1.0 + true_fatigue).sqrt() - 1.0;
    }

    let normalized = true_fatigue / staple;

    let faved = if normalized > 1.0 {
        normalized.sqrt()
    } else {
        normalized * normalized
    };
    faved * staple
}
