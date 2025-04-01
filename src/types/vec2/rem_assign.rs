use std::ops::RemAssign;

use crate::{Number, Vec2};

impl RemAssign<Vec2> for Vec2 {
    #[inline]
    fn rem_assign(&mut self, rhs: Self) {
        self.x.rem_assign(rhs.x);
        self.y.rem_assign(rhs.y);
    }
}
impl<T: Into<Number> + Clone> RemAssign<T> for Vec2 {
    #[inline]
    fn rem_assign(&mut self, rhs: T) {
        self.x.rem_assign(Into::<Number>::into(rhs.clone()));
        self.y.rem_assign(Into::<Number>::into(rhs));
    }
}
