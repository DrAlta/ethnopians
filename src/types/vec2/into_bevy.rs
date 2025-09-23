use crate::Number;

impl Into<bevy::math::Vec2> for crate::Vec2 {
    fn into(self) -> bevy::math::Vec2 {
        bevy::math::Vec2 {
            x: Into::<f32>::into(&self.x),
            y: Into::<f32>::into(&self.y),
        }
    }
}

impl From<bevy::math::Vec2> for crate::Vec2 {
    fn from(value: bevy::math::Vec2) -> Self {
        Self {
            x: Into::<Number>::into(value.x),
            y: Into::<Number>::into(value.y),
        }
    }
}
