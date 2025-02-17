use crate::Number;

impl Into<bevy::math::Vec2> for crate::Vec2 {
    fn into(self) -> bevy::math::Vec2 {
        bevy::math::Vec2 {
            x: self.x as f32,
            y: self.y as f32,
        }
    }
}

impl From<bevy::math::Vec2> for crate::Vec2 {
    fn from(value: bevy::math::Vec2) -> Self {
        Self {
            x: value.x as Number,
            y: value.y as Number,
        }
    }
}
