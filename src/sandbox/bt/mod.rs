#![allow(dead_code)]

mod blackboard;
pub use blackboard::{Blackboard, Variable};
pub mod cpu;
mod stack_item;
pub use stack_item::StackItem;
mod status;
pub use status::Status;
mod thread;
pub use thread::Thread;


type ActionId = usize;
type ExecutionToken = usize;
type World = (bool, bool);

