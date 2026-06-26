mod matrix;
use std::array;

pub use matrix::{MatMul, Matrix};

pub fn rate<const Q: usize>(
    qualities: &Matrix<1, Q>,
    prefs: &Matrix<1, Q>,
    avg: &Matrix<1, Q>,
    culture: &Matrix<Q, Q>,
) -> f32 {
    let mut iter = qualities.comp_div(avg)[0]
        .into_iter()
        .map(|x| (x - 1.0).tanh() + 1.0);
    let normalized_quals: [[f32; Q]; 1] = [array::from_fn(|_| iter.next().unwrap())];

    let subjects_ideals = normalized_quals.mat_mul(culture);

    let adjusted_ideals = subjects_ideals.scaler_add(0.001);

    let a = normalized_quals.comp_sub(&adjusted_ideals);
    let b = a.comp_abs();
    let c = adjusted_ideals.comp_sub(&b);
    let d = c.comp_div(&adjusted_ideals);
    let e = d.comp_mul(prefs);

    let sum_ratings: f32 = e[0].into_iter().sum();

    let sum_personal_weights: f32 = prefs[0].iter().sum();
    sum_ratings / sum_personal_weights
}
