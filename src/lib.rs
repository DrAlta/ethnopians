#![feature(btree_cursors)]
mod types;
pub use types::{ActionID, ActorID, Desire, TimeIndex};

#[cfg(not(feature = "macro"))]
mod vec2;
#[cfg(not(feature = "macro"))]
pub use vec2::{Vec2, vec2};

#[cfg(feature = "macro")]
pub use macroquad::math::{Vec2, vec2};

pub mod behavior_tree;
mod brothel;
pub mod combat;
pub mod formation;
pub mod sandbox;
pub mod social_sim;
pub mod steering;
mod trauma;

pub mod sqrt;

pub type Number = f64;

