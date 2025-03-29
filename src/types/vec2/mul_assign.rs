use std::ops::MulAssign;

use crate::{Number, Vec2};

impl MulAssign<Vec2> for Vec2 {
    #[inline]
    fn mul_assign(&mut self, rhs: Self) {
        self.x.mul_assign(rhs.x);
        self.y.mul_assign(rhs.y);
    }
}
impl<T:Into<Number> + Clone> MulAssign<T> for Vec2 {
    #[inline]
    fn mul_assign(&mut self, rhs: T) {
        self.x.mul_assign(Into::<Number>::into(rhs.clone()));
        self.y.mul_assign(Into::<Number>::into(rhs));
    }
}
