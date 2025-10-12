#![feature(box_into_inner)]
#![feature(btree_cursors)]
pub mod anger;
pub mod behavior_tree;
pub mod broadphase;
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
pub mod preferances;
pub mod probing;
pub mod pubsub;
mod ring;
pub use ring::{ring, Box};
pub mod sandbox;
pub mod seven_emotions;
//pub mod social_sim;//! this is supersecced by the ensemblage crate
pub mod trauma;
pub mod types;
pub use types::{vec2, Vec2};
pub mod util;

pub type Number = ordered_f32::OrderedF32;
pub const IOTA: Number = ordered_f32::OrderedF32(0.0000001);
