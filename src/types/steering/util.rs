pub fn radians_to_u8(radians: f32) -> u8 {
    // Normalize radians to the range [0, 2*pi)
    let normalized = radians.rem_euclid(2.0 * std::f32::consts::PI);
    // Map [0, 2*pi) to [0, 255]
    let u8_value = (normalized / (2.0 * std::f32::consts::PI) * 255.0).round() as u8;
    u8_value
}

pub fn u8_to_radians(value: u8) -> f32 {
    let degrees = value as f32;
    degrees * std::f32::consts::PI / 128.0
}
