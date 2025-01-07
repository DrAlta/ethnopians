#![allow(dead_code)]

mod blackboard;
use std::collections::HashMap;

pub use blackboard::{Blackboard, Variable};
pub mod cpu;
mod hermit;
pub use hermit::get_hermit_behavoir_tree;
mod instruction;
pub use instruction::Instruction;
pub mod parser;
mod stack_item;
pub use stack_item::StackItem;
mod status;
pub use status::Status;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum InpulseId {
    Act1,
    Act2,
    Act3,
}
type ExecutionToken = String;
type World = (bool, bool);
type Thread = Instruction;
type TreePool = HashMap<ExecutionToken, Thread>;
type ItemId = String;