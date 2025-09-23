use crate::Number;

pub fn intersect(
    a_min_x: &Number,
    a_min_y: &Number,
    a_max_x: &Number,
    a_max_y: &Number,
    b_min_x: &Number,
    b_min_y: &Number,
    b_max_x: &Number,
    b_max_y: &Number,
) -> bool {
    a_max_y > b_min_y && b_max_y > a_min_y && a_max_x > b_min_x && b_max_x > a_min_x
}
