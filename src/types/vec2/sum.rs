use std::{iter::Sum, ops::Add};

use crate::Vec2;

impl Sum for Vec2 {
    #[inline]
    fn sum<I>(iter: I) -> Self
    where
        I: Iterator<Item = Self>,
    {
        iter.fold(Self::ZERO, Self::add)
    }
}
