#![allow(dead_code)]

mod blackboard;
pub use blackboard::{Blackboard, Variable};
pub mod cpu;
mod node;
pub use node::Node;
mod stack_item;
pub use stack_item::StackItem;
mod status;
pub use status::Status;


type ActionId = usize;
type ReturnPointer = usize;
type World = (bool, bool);

