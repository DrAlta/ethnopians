#![feature(btree_cursors)]
mod types;
pub use types::{ActionID, ActorID, Desire, Steering, radians_to_u8, u8_to_radians, TimeIndex};

#[cfg(not(feature = "macroquad"))]
pub use types::{Vec2, vec2};

#[cfg(feature = "macroquad")]
pub use macroquad::math::{Vec2, vec2};

pub mod behavior_tree;
mod brothel;
pub mod combat;
pub mod formation;
pub mod mate_resource_allocation;
pub mod sandbox;
pub mod social_sim;
mod trauma;

pub mod sqrt;

pub type Number = f64;

