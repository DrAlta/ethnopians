use geometwo::vec2;
use qol::{AsA, logy};

use crate::{Number, Vec2};

pub fn columns(a: Vec2, b: Vec2, num_units: usize, unit_space: Number) -> Vec<Vec2> {
    let len = a.distance(&b);
    let columns = (len / unit_space).floor().max(Number::ONE);
    let column_spaceing = len / columns;
    logy!(
        "trace",
        "len:{len} columns:{columns} column_spaceing:{column_spaceing} c:{}",
        ((&a + &b) * 0.5_f32.as_a::<Number>())
    );
    let left_vector = (&b - &a).normalized() * column_spaceing;
    //print ln!("a:{a} b:{b} c:{}", ((a + b) * 0.5));
    let center = (&a + &b) * 0.5_f32.as_a::<Number>();
    let mut units = num_units;
    let mut ret = Vec::new();
    while units != 0 {
        let mut new = fill_row(center.clone(), columns.as_a::<usize>(), left_vector.clone(), units);
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

    let mut center = &center1
        - if units <= columns && is_odd(units) {
            Vec2::ZERO
        } else {
            &left_vector * 0.5_f32.as_a::<Number>()
        };

    while used < units {
        if left {
            left = false;
            let target = &center - (&left_vector * idx.as_a::<Number>()) + (&back * row.as_a::<Number>());
            if space_free(target.clone()) {
                ret.push(target.clone());
                used += 1;
            }
        } else {
            left = true;
            let target = &center + (&left_vector * idx.as_a::<Number>()) + (&back * row.as_a::<Number>());
            if space_free(target.clone()) {
                ret.push(target.clone());
                used += 1;
            }
        };
        idx += 1;
        if idx >= columns {
            idx = 0;
            row += 1;
            let units_left = units - used;
            center = &center1
                - if units_left <= columns && is_odd(units_left) {
                    Vec2::ZERO
                } else {
                    &left_vector * 0.5.as_a::<Number>()
                };
            logy!("trace", "center:{center}");
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
