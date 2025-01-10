#![allow(dead_code)]

mod blackboard;
use std::collections::HashMap;

pub use blackboard::{Blackboard, Variable};
mod correct;
pub use correct::Corrent;
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
pub type ThreadName = String;
pub type ExecutionToken = ThreadName;
pub type ExecutionPointer = (ExecutionToken, usize);
pub type Thread = Vec<Instruction>;
pub type TreePool = HashMap<ThreadName, Thread>;
pub type ItemId = String;
