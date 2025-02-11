use crate::{vec2, Number, Vec2};

pub fn columns(a: Vec2, b: Vec2, num_units: usize, unit_space: Number) -> Vec<Vec2> {
    let len = a.distance(b);
    let columns = (len / unit_space).floor().max(1.0);
    let column_spaceing = len / columns;
    println!(
        "len:{len} columns:{columns} column_spaceing:{column_spaceing} c:{}",
        ((a + b) * 0.5)
    );
    let left_vector = (b - a).normalize() * column_spaceing;
    //println!("a:{a} b:{b} c:{}", ((a + b) * 0.5));
    let center = (a + b) * 0.5;
    let mut units = num_units;
    let mut ret = Vec::new();
    while units != 0 {
        let mut new = fill_row(center, columns as usize, left_vector, units);
        if new.len() > units {
            units = 0;
        } else {
            units -= new.len();
        };
        ret.append(&mut new);
    }
    ret
}

fn fill_row(center1: Vec2, columns: usize, left_vector: Vec2, units: usize) -> Vec<Vec2> {
    let mut left: bool = true;
    let back = vec2(left_vector.y, -left_vector.x);
    let mut idx = 0;
    let mut row = 0;
    let mut used = 0;
    let mut ret = Vec::new();

    let mut center = center1
        - if units <= columns && is_odd(units) {
            Vec2::ZERO
        } else {
            left_vector * 0.5
        };

    while used < units {
        if left {
            left = false;
            let target = center - (left_vector * idx as f32) + (back * row as f32);
            if space_free(target) {
                ret.push(target);
                used += 1;
            }
        } else {
            left = true;
            let target = center + (left_vector * idx as f32) + (back * row as f32);
            if space_free(target) {
                ret.push(target);
                used += 1;
            }
        };
        idx += 1;
        if idx >= columns {
            idx = 0;
            row += 1;
            let units_left = units - used;
            center = center1
                - if units_left <= columns && is_odd(units_left) {
                    Vec2::ZERO
                } else {
                    left_vector * 0.5
                };
            println!("c:{center}");
        }
    }
    ret
}

fn space_free(_: Vec2) -> bool {
    true
}
fn is_odd(selfie: usize) -> bool {
    selfie & 1 != 0
}
