use super::{Command, EntityId, Location, World};
/// we need to keep calling this until is returns an empty Vec
pub fn collide(a: EntityId, b: EntityId, world: &World) -> Vec<Command> {
    let empty = Vec::new();
    let Some(Location::World { x: ax, y: ay }) = world.get_location(&a) else {
        return empty;
    };
    let Some(Location::World { x: bx, y: by }) = world.get_location(&b) else {
        return empty;
    };

    let Some((aw, ah)) = world.get_size(&a) else {
        return empty;
    };
    let Some((bw, bh)) = world.get_size(&b) else {
        return empty;
    };

    let Some(x_overhang) = (if ax < bx {
        one_d_collide(*ax, *aw, *bx, *bw)
    } else {
        one_d_collide(*bx, *bw, *ax, *aw)
    }) else {
        return empty;
    };
    let Some(y_overhang) = (if ay < by {
        one_d_collide(*ay, *ah, *by, *bh)
    } else {
        one_d_collide(*by, *bh, *ay, *ah)
    }) else {
        return empty;
    };

    let new_x = if x_overhang > 0.0 { bx - aw } else { bx + bw };
    let new_y = if y_overhang > 0.0 { by - ah } else { by + bh };

    if x_overhang.abs() < y_overhang.abs() {
        vec![Command::SetLocation {
            agent_id: a,
            loc: Location::World { x: *ax, y: new_y },
        }]
    } else {
        vec![Command::SetLocation {
            agent_id: a,
            loc: Location::World { x: new_x, y: *ay },
        }]
    }
}

pub fn one_d_collide(min: f32, min_w: f32, max: f32, max_w: f32) -> Option<f32> {
    let max_right = max + max_w;
    if min > max_right {
        return None;
    }
    let min_right = min + min_w;

    // this handles id min is inside max
    if min > max && min_right < max_right {
        let min_mid = min + min_right;
        let max_mid = max + max_right;
        return Some(if min_mid < max_mid {
            max - min_w
        } else {
            max_right
        });
    }

    //ok min isn't in side max so look at which side of b has more of a sting out of it
    let left = max - min;
    let right = min_right - max_right;
    if left > 0.0 && left > right {
        Some(-left)
    } else if right > 0.0 {
        Some(right)
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn one_d_collid_left_test() {
        let x = one_d_collide(5.0, 10.0, 10.0, 1.0);
        assert_eq!(Some(-5.0), x);
    }
    #[test]
    pub fn one_d_collid_pen_test() {
        let x = one_d_collide(11.0, 10.0, 10.0, 20.0);
        assert_eq!(Some(0.0), x);
    }
}
