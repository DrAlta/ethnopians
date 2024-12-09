use std::{collections::BTreeMap, ops::Add};

mod degrees;
use degrees::Degrees;

mod util;
pub use util::{lerp, radians_to_u8, u8_to_radians};

pub struct Steering(BTreeMap<u8, f32>);
impl Steering {
    pub fn new() -> Self {
        Self(BTreeMap::new())
    }
    pub fn get<T: Degrees>(&self, direction: T) -> Option<f32> {
        let direction = direction.degrees();
        let x = self
            .0
            .upper_bound(std::collections::Bound::Included(&direction));
        let (prev_key, prev) = if let Some(a) = x.peek_prev() {
            a
        } else {
            let a = self.0.last_key_value()?;
            a
        };
        let (next_key, next) = if let Some(b) = x.peek_next() {
            b
        } else {
            let b = self.0.first_key_value()?;
            b
        };

        // Calculate interpolation factor
        let a = (direction as f32 - *prev_key as f32).rem_euclid(256.0);

        let b = (*next_key as f32 - *prev_key as f32).rem_euclid(256.0);

        let t = a / b;

        // Interpolate between the two closest values
        Some(lerp(*prev, *next, t))
    }
    pub fn max<T: Degrees>(&self) -> Option<f32> {
        Some(*(&self.0).values().max_by(|a, b| a.total_cmp(b))?)
    }
}
impl<T: Into<BTreeMap<u8, f32>>> From<T> for Steering {
    fn from(value: T) -> Self {
        Self(value.into())
    }
}

impl Add for &Steering {
    type Output = Steering;

    fn add(self, rhs: Self) -> Self::Output {
        let keys: Vec<&u8> = self.0.keys().chain(rhs.0.keys()).collect();
        let mut map = BTreeMap::new();
        for key in keys {
            let value = match (self.get(key), rhs.get(key)) {
                (None, None) => continue,
                (None, Some(x)) => x,
                (Some(x), None) => x,
                (Some(a), Some(b)) => a + b,
            };
            map.insert(*key, value);
        }
        Steering(map)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn over_test() {
        let map = Steering::from(BTreeMap::from([(0, 10.0), (5, 5.0), (9, 10.0)]));

        assert_eq!(10.0, map.get(20).unwrap());
    }
    #[test]
    pub fn under_test() {
        let map = Steering::from(BTreeMap::from([(3, 10.0), (5, 5.0), (9, 10.0)]));

        assert_eq!(10.0, map.get(1).unwrap());
    }
    #[test]
    pub fn mid_test() {
        let map = Steering::from(BTreeMap::from([(3, 5.0), (5, 10.0), (100, 10.0)]));

        assert_eq!(10.0, map.get(50).unwrap());
    }
}
