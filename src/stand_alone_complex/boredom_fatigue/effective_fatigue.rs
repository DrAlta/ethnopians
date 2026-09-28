use crate::stand_alone_complex::boredom_fatigue::{favorite_fatigue, staple_fatigue};

#[derive(Debug)]
pub enum Liking {
    None,
    Staple(f64),
    Favoite(f64),
}
pub fn effective_fatigue(true_fatigue: f64, liking: &Liking) -> f64 {
    if true_fatigue < 0.0 {
        return true_fatigue;
    }
    match liking {
        Liking::None => true_fatigue,
        Liking::Staple(staple) => staple_fatigue(true_fatigue, *staple),
        Liking::Favoite(favorite) => favorite_fatigue(true_fatigue, *favorite),
    }
}
