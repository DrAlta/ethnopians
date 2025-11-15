#![feature(box_into_inner)]
#![feature(btree_cursors)]
use std::sync::LazyLock;
pub mod anger;
pub mod behavior_tree;
mod brothel;
pub mod combat;
pub mod emotional_dysregulation;
pub mod formation;
pub mod general_specific_affinity;
#[cfg(feature = "gossip")]
pub mod gossip;
pub mod kill_share;
pub mod mate;
pub mod multi_party_dialogue;
pub mod museum;
pub mod preferances;
pub mod probing;
pub mod pubsub;
mod ring;
pub use ring::{ring, Box};
pub mod sandbox;
pub mod seven_emotions;
//pub mod social_sim;//! this is supersecced by the ensemblage crate
pub mod stock_market_fractal;
pub mod trauma;
pub mod types;
pub mod util;

pub type Vec2 = geometwo::Vector2;
pub type Number = geometwo::Number;
pub static IOTA: LazyLock<Number> = LazyLock::new(||0.0000001.into());


