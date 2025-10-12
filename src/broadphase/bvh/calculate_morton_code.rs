use crate::{
    broadphase::bvh::{expand_bits, MortenCode},
    Number,
};

pub fn calculate_morton_code(
    x: &Number,
    y: &Number,
    min_x: &Number,
    min_y: &Number,
    max_x: &Number,
    max_y: &Number,
) -> MortenCode {
    let x2 = ((x - min_x) / (max_x - min_x) * 2_f32.powi(16_i32)).floor() as u16;
    let y2 = ((y - min_y) / (max_y - min_y) * 2_f32.powi(16_i32)).floor() as u16;

    expand_bits(x2) | (expand_bits(y2) << 1)
}
