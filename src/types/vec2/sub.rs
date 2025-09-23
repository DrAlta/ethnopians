use std::ops::Sub;

use crate::{Number, Vec2};

impl Sub for Vec2 {
    type Output = Vec2;
    #[inline]
    fn sub(self, rhs: Self) -> Vec2 {
        (&self).sub(&rhs)
    }
}
impl Sub for &Vec2 {
    type Output = Vec2;
    #[inline]
    fn sub(self, rhs: Self) -> Vec2 {
        Vec2 {
            x: self.x.sub(rhs.x),
            y: self.y.sub(rhs.y),
        }
    }
}

impl<T: Into<Number> + Clone> Sub<T> for Vec2 {
    type Output = Self;
    #[inline]
    fn sub(self, rhs: T) -> Self {
        Self {
            x: self.x.sub(Into::<Number>::into(rhs.clone())),
            y: self.y.sub(Into::<Number>::into(rhs)),
        }
    }
}
