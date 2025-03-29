use std::ops::SubAssign;

use crate::{Number, Vec2};

impl SubAssign<Vec2> for Vec2 {
    #[inline]
    fn sub_assign(&mut self, rhs: Vec2) {
        self.x.sub_assign(rhs.x);
        self.y.sub_assign(rhs.y);
    }
}


impl<T: Into<Number> + Clone> SubAssign<T> for Vec2 {
    #[inline]
    fn sub_assign(&mut self, rhs: T) {
        self.x.sub_assign(Into::<Number>::into(rhs.clone()));
        self.y.sub_assign(Into::<Number>::into(rhs));
    }
}

