use std::ops::Rem;

use crate::{Number, Vec2};

impl Rem<Vec2> for Vec2 {
    type Output = Self;
    #[inline]
    fn rem(self, rhs: Self) -> Self {
        Self {
            x: self.x.rem(rhs.x),
            y: self.y.rem(rhs.y),
        }
    }
}
impl<T: Into<Number> + Clone> Rem<T> for Vec2 {
    type Output = Self;
    #[inline]
    fn rem(self, rhs: T) -> Self {
        Self {
            x: self.x.rem(Into::<Number>::into(rhs.clone())),
            y: self.y.rem(Into::<Number>::into(rhs)),
        }
    }
}
