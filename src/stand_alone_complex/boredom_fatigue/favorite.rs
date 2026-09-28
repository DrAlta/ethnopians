pub fn favorite_fatigue(true_fatigue: f64, favorite: f64) -> f64 {
    if true_fatigue < 0.0 {
        return true_fatigue;
    }
    if favorite == 0.0 {
        let h = 1.0 - 0.5_f64.powf(true_fatigue);
        return h + (2.0 * h * (h / 2.0));
    }
    let fav = favorite.min(2.0);

    let normalized = true_fatigue / fav;

    if normalized > 1.0 {
        let h = 1.0 - 0.5_f64.powf(normalized - 1.0);
        let diff = 2.0 - fav;

        fav + (diff * h)
    } else {
        normalized * normalized * fav
    }
}
