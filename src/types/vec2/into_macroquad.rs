use crate::Number;

impl Into<macroquad::math::Vec2> for crate::Vec2 {
    fn into(self) -> macroquad::math::Vec2 {
        macroquad::math::Vec2 {
            x: self.x.into(),
            y: self.y.into(),
        }
    }
}

impl From<macroquad::math::Vec2> for crate::Vec2 {
    fn from(value: macroquad::math::Vec2) -> Self {
        Self {
            x: value.x.into(),
            y: value.y.into(),
        }
    }
}
