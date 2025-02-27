use crate::Number;

pub fn lerp(a: Number, b: Number, t: Number) -> Number {
    a + t * (b - a)
}
