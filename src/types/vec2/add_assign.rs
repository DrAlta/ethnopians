use std::ops::AddAssign;

use crate::{Number, Vec2};

impl AddAssign<Vec2> for Vec2 {
    #[inline]
    fn add_assign(&mut self, rhs: Self) {
        self.x.add_assign(rhs.x);
        self.y.add_assign(rhs.y);
    }
}

impl<T: Into<Number> + Clone> AddAssign<T> for Vec2 {
    #[inline]
    fn add_assign(&mut self, rhs: T) {
        self.x.add_assign(Into::<Number>::into(rhs.clone()));
        self.y.add_assign(Into::<Number>::into(rhs));
    }
}
