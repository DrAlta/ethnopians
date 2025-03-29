use std::ops::Add;

use crate::{Number, Vec2};

impl Add<Vec2> for Vec2 {
    type Output = Self;
    #[inline]
    fn add(self, rhs: Self) -> Self {
        Self {
            x: self.x.add(rhs.x),
            y: self.y.add(rhs.y),
        }
    }
}


impl<T: Into<Number> + Clone> Add<T> for Vec2 {
    type Output = Self;
    #[inline]
    fn add(self, rhs: T) -> Self {
        Self {
            x: self.x.add(Into::<Number>::into(rhs.clone())),
            y: self.y.add(Into::<Number>::into(rhs)),
        }
    }
}
