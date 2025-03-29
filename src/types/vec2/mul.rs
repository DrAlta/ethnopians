use std::ops::Mul;

use crate::{Number, Vec2};

impl Mul<Vec2> for Vec2 {
    type Output = Self;
    #[inline]
    fn mul(self, rhs: Self) -> Self {
        Self {
            x: self.x.mul(rhs.x),
            y: self.y.mul(rhs.y),
        }
    }
}
impl<T:Into<Number> + Clone> Mul<T> for Vec2 {
    type Output = Self;
    #[inline]
    fn mul(self, rhs: T) -> Self {
        Self {
            x: self.x.mul(Into::<Number>::into(rhs.clone())),
            y: self.y.mul(Into::<Number>::into(rhs)),
        }
    }
}
