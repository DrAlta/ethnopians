/// this is for calculating first inpressions
/// you calc some `diffrance_in_personalities`
/// then `let first_impression = how_well_their_interaction_when * zero2one_infinity_to_zero_throught_point(diffrance_in_personalities, 0.01, max_diffrance_in_personalities);`
use crate::Number;

pub fn calc_zero2one_infinity_to_zero_coefficient(
    target_input: Number,
    target_output: Number,
) -> Number {
    (Number::ONE - target_output) / (target_output * target_input)
}
pub fn zero2one_infinity_to_zero_with_coefficient(i: Number, coefficient: Number) -> Number {
    Number::ONE / ((i * coefficient) + Number::ONE)
}
pub fn zero2one_infinity_to_zero_throught_point(
    i: Number,
    target_input: Number,
    target_output: Number,
) -> Number {
    let coefficient = calc_zero2one_infinity_to_zero_coefficient(target_output, target_input);
    zero2one_infinity_to_zero_with_coefficient(i, coefficient)
}
