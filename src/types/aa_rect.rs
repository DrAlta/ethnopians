use crate::Number;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct AARect {
    pub min_x: Number,
    pub min_y: Number,
    pub width: Number,
    pub height: Number,
}
impl AARect {
    pub fn new(min_x: Number, min_y: Number, width: Number, height: Number) -> Self {
        Self {
            min_x,
            min_y,
            width,
            height,
        }
    }
    pub fn get_min_x(&self) -> Number {
        self.min_x
    }
    pub fn get_min_y(&self) -> Number {
        self.min_y
    }
    pub fn inside(&self, x: Number, y: Number) -> bool {
        x >= self.min_x
            && x <= self.min_x + self.width
            && y >= self.min_y
            && y <= self.min_y + self.height
    }
}

