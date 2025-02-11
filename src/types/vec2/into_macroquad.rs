use crate::Number;

impl Into<macroquad::math::Vec2> for crate::Vec2 {
    fn into(self) -> macroquad::math::Vec2 {
        macroquad::math::Vec2{
            x: self.x as f32,
            y: self.y as f32
        }
    }
}

impl From<macroquad::math::Vec2> for crate::Vec2 {
    fn from(value: macroquad::math::Vec2) -> Self {
        Self { 
            x: value.x as Number, y: value.y as Number
        }
    }
}