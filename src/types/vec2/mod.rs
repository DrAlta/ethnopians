mod vec2;
pub use vec2::{vec2, Vec2};

#[cfg(feature = "bevy")]
mod into_bevy;

#[cfg(feature = "macroquad")]
mod into_macroquad;

mod add;
mod add_assign;
mod div;
mod div_assign;
mod mul;
mod mul_assign;
mod sub;
mod sub_assign;
mod rem;
mod rem_assign;

mod sum;

mod display;