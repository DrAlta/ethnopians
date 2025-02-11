mod vec2;
pub use vec2::{vec2, Vec2};

#[cfg(feature = "bevy")]
mod into_bevy;

#[cfg(feature = "macroquad")]
mod into_macroquad;
