use crate::Number;

pub trait Consts{
    const ONE: Self;
}

impl Consts for Number {
    const ONE: Self = 1.0;

}