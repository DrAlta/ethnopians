#![feature(box_into_inner)]
#![feature(btree_cursors)]
pub mod behavior_tree;
mod brothel;
pub mod combat;
pub mod formation;
pub mod kill_share;
pub mod mate;
pub mod preferances;
pub mod sandbox;
//pub mod social_sim;//! this is supersecced by the ensemblage crate
mod trauma;
pub mod types;
pub use types::{vec2, Consts, Vec2};
pub mod util;

pub type Number = f64;
