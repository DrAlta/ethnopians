use std::{
    iter::Sum,
    ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Rem, RemAssign, Sub, SubAssign},
};

use crate::Number;

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Vec2 {
    pub x: Number,
    pub y: Number,
}
impl Vec2 {
    pub const ZERO: Vec2 = Vec2 { x: 0.0, y: 0.0 };
    pub fn normalize(self) -> Self {
        self.mul(self.length().recip())
    }
    pub fn distance(self, rhs: Self) -> Number {
        (self - rhs).length()
    }
    pub fn dot(self, rhs: Self) -> Number {
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

impl DivAssign<Vec2> for Vec2 {
    #[inline]
    fn div_assign(&mut self, rhs: Self) {
        self.x.div_assign(rhs.x);
        self.y.div_assign(rhs.y);
    }
}

impl Div<f32> for Vec2 {
    type Output = Self;
    #[inline]
    fn div(self, rhs: f32) -> Self {
        Self {
            x: self.x.div(rhs as Number),
            y: self.y.div(rhs as Number),
        }
    }
}
impl Div<f64> for Vec2 {
    type Output = Self;
    #[inline]
    fn div(self, rhs: f64) -> Self {
        Self {
            x: self.x.div(rhs),
            y: self.y.div(rhs),
        }
    }
}

impl DivAssign<f32> for Vec2 {
    #[inline]
    fn div_assign(&mut self, rhs: f32) {
        self.x.div_assign(rhs as Number);
        self.y.div_assign(rhs as Number);
    }
}

impl DivAssign<f64> for Vec2 {
    #[inline]
    fn div_assign(&mut self, rhs: f64) {
        self.x.div_assign(rhs);
        self.y.div_assign(rhs);
    }
}

impl Div<Vec2> for f32 {
    type Output = Vec2;
    #[inline]
    fn div(self, rhs: Vec2) -> Vec2 {
        Vec2 {
            x: (self as Number).div(rhs.x),
            y: (self as Number).div(rhs.y),
        }
    }
}

impl Div<Vec2> for f64 {
    type Output = Vec2;
    #[inline]
    fn div(self, rhs: Vec2) -> Vec2 {
        Vec2 {
            x: self.div(rhs.x),
            y: self.div(rhs.y),
        }
    }
}

impl Mul<Vec2> for Vec2 {
    type Output = Self;
    #[inline]
    fn mul(self, rhs: Self) -> Self {
        Self {
            x: self.x.mul(rhs.x),
            y: self.y.mul(rhs.y),
        }
    }
}

impl MulAssign<Vec2> for Vec2 {
    #[inline]
    fn mul_assign(&mut self, rhs: Self) {
        self.x.mul_assign(rhs.x);
        self.y.mul_assign(rhs.y);
    }
}

impl Mul<f32> for Vec2 {
    type Output = Self;
    #[inline]
    fn mul(self, rhs: f32) -> Self {
        Self {
            x: self.x.mul(rhs as Number),
            y: self.y.mul(rhs as Number),
        }
    }
}

impl Mul<f64> for Vec2 {
    type Output = Self;
    #[inline]
    fn mul(self, rhs: f64) -> Self {
        Self {
            x: self.x.mul(rhs),
            y: self.y.mul(rhs),
        }
    }
}

impl MulAssign<f32> for Vec2 {
    #[inline]
    fn mul_assign(&mut self, rhs: f32) {
        self.x.mul_assign(rhs as Number);
        self.y.mul_assign(rhs as Number);
    }
}

impl MulAssign<f64> for Vec2 {
    #[inline]
    fn mul_assign(&mut self, rhs: f64) {
        self.x.mul_assign(rhs);
        self.y.mul_assign(rhs);
    }
}

impl Mul<Vec2> for f32 {
    type Output = Vec2;
    #[inline]
    fn mul(self, rhs: Vec2) -> Vec2 {
        Vec2 {
            x: (self as Number).mul(rhs.x),
            y: (self as Number).mul(rhs.y),
        }
    }
}

impl Mul<Vec2> for f64 {
    type Output = Vec2;
    #[inline]
    fn mul(self, rhs: Vec2) -> Vec2 {
        Vec2 {
            x: self.mul(rhs.x),
            y: self.mul(rhs.y),
        }
    }
}

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

impl AddAssign<Vec2> for Vec2 {
    #[inline]
    fn add_assign(&mut self, rhs: Self) {
        self.x.add_assign(rhs.x);
        self.y.add_assign(rhs.y);
    }
}

impl Add<f32> for Vec2 {
    type Output = Self;
    #[inline]
    fn add(self, rhs: f32) -> Self {
        Self {
            x: self.x.add(rhs as Number),
            y: self.y.add(rhs as Number),
        }
    }
}

impl Add<f64> for Vec2 {
    type Output = Self;
    #[inline]
    fn add(self, rhs: f64) -> Self {
        Self {
            x: self.x.add(rhs as Number),
            y: self.y.add(rhs as Number),
        }
    }
}

impl AddAssign<f32> for Vec2 {
    #[inline]
    fn add_assign(&mut self, rhs: f32) {
        self.x.add_assign(rhs as Number);
        self.y.add_assign(rhs as Number);
    }
}

impl AddAssign<f64> for Vec2 {
    #[inline]
    fn add_assign(&mut self, rhs: f64) {
        self.x.add_assign(rhs);
        self.y.add_assign(rhs);
    }
}

impl Add<Vec2> for f32 {
    type Output = Vec2;
    #[inline]
    fn add(self, rhs: Vec2) -> Vec2 {
        Vec2 {
            x: (self as Number).add(rhs.x),
            y: (self as Number).add(rhs.y),
        }
    }
}

impl Add<Vec2> for f64 {
    type Output = Vec2;
    #[inline]
    fn add(self, rhs: Vec2) -> Vec2 {
        Vec2 {
            x: self.add(rhs.x),
            y: self.add(rhs.y),
        }
    }
}

impl Sub<Vec2> for Vec2 {
    type Output = Self;
    #[inline]
    fn sub(self, rhs: Self) -> Self {
        Self {
            x: self.x.sub(rhs.x),
            y: self.y.sub(rhs.y),
        }
    }
}

impl SubAssign<Vec2> for Vec2 {
    #[inline]
    fn sub_assign(&mut self, rhs: Vec2) {
        self.x.sub_assign(rhs.x);
        self.y.sub_assign(rhs.y);
    }
}

impl Sub<f32> for Vec2 {
    type Output = Self;
    #[inline]
    fn sub(self, rhs: f32) -> Self {
        Self {
            x: self.x.sub(rhs as Number),
            y: self.y.sub(rhs as Number),
        }
    }
}

impl Sub<f64> for Vec2 {
    type Output = Self;
    #[inline]
    fn sub(self, rhs: f64) -> Self {
        Self {
            x: self.x.sub(rhs as Number),
            y: self.y.sub(rhs as Number),
        }
    }
}

impl SubAssign<f32> for Vec2 {
    #[inline]
    fn sub_assign(&mut self, rhs: f32) {
        self.x.sub_assign(rhs as Number);
        self.y.sub_assign(rhs as Number);
    }
}

impl SubAssign<f64> for Vec2 {
    #[inline]
    fn sub_assign(&mut self, rhs: f64) {
        self.x.sub_assign(rhs as Number);
        self.y.sub_assign(rhs as Number);
    }
}

impl Sub<Vec2> for f32 {
    type Output = Vec2;
    #[inline]
    fn sub(self, rhs: Vec2) -> Vec2 {
        Vec2 {
            x: (self as Number).sub(rhs.x),
            y: (self as Number).sub(rhs.y),
        }
    }
}

impl Sub<Vec2> for f64 {
    type Output = Vec2;
    #[inline]
    fn sub(self, rhs: Vec2) -> Vec2 {
        Vec2 {
            x: (self as Number).sub(rhs.x),
            y: (self as Number).sub(rhs.y),
        }
    }
}

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

impl RemAssign<Vec2> for Vec2 {
    #[inline]
    fn rem_assign(&mut self, rhs: Self) {
        self.x.rem_assign(rhs.x);
        self.y.rem_assign(rhs.y);
    }
}

impl Rem<f32> for Vec2 {
    type Output = Self;
    #[inline]
    fn rem(self, rhs: f32) -> Self {
        Self {
            x: self.x.rem(rhs as Number),
            y: self.y.rem(rhs as Number),
        }
    }
}

impl Rem<f64> for Vec2 {
    type Output = Self;
    #[inline]
    fn rem(self, rhs: f64) -> Self {
        Self {
            x: self.x.rem(rhs),
            y: self.y.rem(rhs),
        }
    }
}

impl RemAssign<f32> for Vec2 {
    #[inline]
    fn rem_assign(&mut self, rhs: f32) {
        self.x.rem_assign(rhs as Number);
        self.y.rem_assign(rhs as Number);
    }
}

impl RemAssign<f64> for Vec2 {
    #[inline]
    fn rem_assign(&mut self, rhs: f64) {
        self.x.rem_assign(rhs as Number);
        self.y.rem_assign(rhs as Number);
    }
}

impl Rem<Vec2> for f32 {
    type Output = Vec2;
    #[inline]
    fn rem(self, rhs: Vec2) -> Vec2 {
        Vec2 {
            x: (self as Number).rem(rhs.x),
            y: (self as Number).rem(rhs.y),
        }
    }
}

impl Rem<Vec2> for f64 {
    type Output = Vec2;
    #[inline]
    fn rem(self, rhs: Vec2) -> Vec2 {
        Vec2 {
            x: (self as Number).rem(rhs.x),
            y: (self as Number).rem(rhs.y),
        }
    }
}
/*
impl AsRef<[Number; 2]> for Vec2 {
    #[inline]
    fn as_ref(&self) -> &[Number; 2] {
        unsafe { &*(self as *const Vec2 as *const [Number; 2]) }
    }
}

impl AsMut<[f32; 2]> for Vec2 {
    #[inline]
    fn as_mut(&mut self) -> &mut [f32; 2] {
        unsafe { &mut *(self as *mut Vec2 as *mut [f32; 2]) }
    }
}*/

impl Sum for Vec2 {
    #[inline]
    fn sum<I>(iter: I) -> Self
    where
        I: Iterator<Item = Self>,
    {
        iter.fold(Self::ZERO, Self::add)
    }
}
impl std::fmt::Display for Vec2 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(p) = f.precision() {
            write!(f, "[{:.*}, {:.*}]", p, self.x, p, self.y)
        } else {
            write!(f, "[{}, {}]", self.x, self.y)
        }
    }
}
