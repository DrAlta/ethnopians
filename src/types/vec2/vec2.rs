use std::
    ops::Mul
;

use crate::Number;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Copy, Hash)]
pub struct Vec2 {
    pub x: Number,
    pub y: Number,
}
impl Vec2 {
    pub const ZERO: Vec2 = Vec2 { x: Number::ZERO, y: Number::ZERO };
    pub fn normalize(self) -> Self {
        self.mul(self.length().recip())
    }
    pub fn distance(self, rhs: Self) -> Number {
        (self - rhs).length()
    }
    pub fn dot(self, rhs: &Self) -> Number {
        (self.x * rhs.x) + (self.y * rhs.y)
    }
    pub fn length_squared(self) -> Number {
        (self.x * self.x) + (self.y * self.y)
    }
    pub fn length(self) -> Number {
        self.length_squared().sqrt()
    }
}

pub fn vec2(x: Number, y: Number) -> Vec2 {
    Vec2 { x, y }
}




/*










    */
