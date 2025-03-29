use crate::Vec2;


impl std::fmt::Display for Vec2 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(p) = f.precision() {
            write!(f, "[{:.*}, {:.*}]", p, self.x, p, self.y)
        } else {
            write!(f, "[{}, {}]", self.x, self.y)
        }
    }
}
