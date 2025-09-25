mod blackboard;
pub use blackboard::{Blackboard, BlackboardValue, Variable};
pub mod behavior_tree;
pub mod forth;
//pub mod forth_parser;

type InpulseId = usize;

pub type BlackboardKey = String;
