#![feature(box_into_inner)]
#![feature(btree_cursors)]
mod types;

pub use types::{radians_to_u8, u8_to_radians, ActionId, ActorId, Desire, Steering, TimeIndex};

pub use types::{vec2, Vec2};

pub mod behavior_tree;
mod brothel;
pub mod combat;
pub mod formation;
pub mod kill_share;
pub mod mate;
pub mod preferances;
pub mod sandbox;
pub mod social_sim;
mod trauma;

pub mod util;

pub type Number = f64;
