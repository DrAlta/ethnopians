use super::{AROMATICS_SIZE, TECHNICAL_SIZE};

mod critique_dish;
mod rate_dish;

pub struct Customer {
    name: &'static str,

    basel_joy_of_eating: f32,

    target_intensity: f32, // Their "Ideal Blandness" sum

    technical_ideal_relative_offsets: [f32; TECHNICAL_SIZE],
    technical_extreme_prefs: [f32; TECHNICAL_SIZE],

    aromatics_ideal_relative_offsets: [f32; AROMATICS_SIZE],
    aromatics_extreme_prefs: [f32; AROMATICS_SIZE],

}
impl Customer{
    pub fn new(
        name: &'static str,

        basel_joy_of_eating: f32,

        target_intensity: f32,

        technical_ideal_relative_offsets: [f32; TECHNICAL_SIZE],
        technical_extreme_prefs: [f32; TECHNICAL_SIZE],

        aromatics_ideal_relative_offsets: [f32; AROMATICS_SIZE],
        aromatics_extreme_prefs: [f32; AROMATICS_SIZE],

    )-> Self {
        Self { name, basel_joy_of_eating, target_intensity, technical_ideal_relative_offsets, technical_extreme_prefs, aromatics_ideal_relative_offsets, aromatics_extreme_prefs }
    }
    pub fn name(&self) -> &str {
        self.name
    }
}