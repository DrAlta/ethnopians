#![allow(dead_code)]

mod blackboard;
use std::collections::HashMap;

pub use blackboard::{Blackboard, Variable};
pub mod cpu;
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
pub type ExecutionToken = String;
pub type World = (bool, bool);
pub type Thread = Instruction;
pub type TreePool = HashMap<ExecutionToken, Thread>;
pub type ItemId = String;
