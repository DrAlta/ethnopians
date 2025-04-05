#![feature(box_into_inner)]
#![feature(btree_cursors)]
pub mod behavior_tree;
mod brothel;
mod bvh;
pub use bvh::Node;
pub mod combat;
pub mod formation;
pub mod kill_share;
pub mod mate;
pub mod preferances;
pub mod sandbox;
pub mod seven_emotions;
//pub mod social_sim;//! this is supersecced by the ensemblage crate
mod trauma;
pub mod types;
pub use types::{vec2, Vec2};
pub mod util;

pub type Number = ordered_f32::OrderedF32;
const IOTA: Number = ordered_f32::OrderedF32(0.0000001);
