use std::ops::Div;

use crate::{Number, Vec2};


impl Div<Vec2> for Vec2 {
    type Output = Self;
    #[inline]
    fn div(self, rhs: Self) -> Self {
        Self {
            x: self.x.div(rhs.x),
            y: self.y.div(rhs.y),
        }
    }
}


impl<T:Into<Number> + Clone> Div<T> for Vec2 {
    type Output = Self;
    #[inline]
    fn div(self, rhs: T) -> Self {
        Self {
            x: self.x.div(Into::<Number>::into(rhs.clone())),
            y: self.y.div(Into::<Number>::into(rhs)),
        }
    }
}
