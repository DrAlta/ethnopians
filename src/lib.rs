#![feature(box_into_inner)]
#![feature(btree_cursors)]
use std::sync::LazyLock;

pub mod sandbox;
//pub mod social_sim;//! this is supersecced by the ensemblage crate
pub mod stand_alone_complex;
pub mod types;
pub mod util;
mod ring;
pub use ring::{ring, Box};

pub type Vec2 = geometwo::Vector2;
pub type Number = geometwo::Number;
pub static IOTA: LazyLock<Number> = LazyLock::new(||0.0000001.into());


