use std::ops::DivAssign;

use crate::{Number, Vec2};

impl DivAssign<Vec2> for Vec2 {
    #[inline]
    fn div_assign(&mut self, rhs: Self) {
        self.x.div_assign(rhs.x);
        self.y.div_assign(rhs.y);
    }
}

impl<T: Into<Number> + Clone> DivAssign<T> for Vec2 {
    #[inline]
    fn div_assign(&mut self, rhs: T) {
        self.x.div_assign(Into::<Number>::into(rhs.clone()));
        self.y.div_assign(Into::<Number>::into(rhs));
    }
}
