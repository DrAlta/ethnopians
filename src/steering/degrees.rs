use super::radians_to_u8;

pub trait Degrees{
    fn degrees(self) -> u8;
}

impl Degrees for u8 {
    fn degrees(self) -> u8 {
        self
    }
}
impl Degrees for &u8 {
    fn degrees(self) -> u8 {
        *self
    }
}
impl Degrees for f32 {
    fn degrees(self) -> u8 {
        radians_to_u8(self)
    }
}
