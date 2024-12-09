#![feature(btree_cursors)]
mod types;
pub use types::{radians_to_u8, u8_to_radians, ActionID, ActorID, Desire, Steering, TimeIndex};

#[cfg(not(feature = "macroquad"))]
pub use types::{vec2, Vec2};

#[cfg(feature = "macroquad")]
pub use macroquad::math::{vec2, Vec2};

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
