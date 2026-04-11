use super::super::{Customer, FlavorVector};

impl Customer {
    pub fn rate_dish(&self, fv: &FlavorVector) -> f32 {
        self.basel_joy_of_eating + self.critique_dish(fv)
    }
}