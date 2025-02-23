use crate::sandbox::ai::{ExecutionPointer, Status};

mod blackboard;
pub use blackboard::{Blackboard, BlackboardValue, Variable};
mod cpu;
pub use cpu::CPU;
/*
mod tick_active_selector;
pub use tick_active_selector::tick_active_selector;
*/
mod instruction;
pub use instruction::Instruction;
mod stack_item;
pub use stack_item::{StackItem, TableGet, TableInterior};
mod tick_action;
pub use tick_action::tick_action;
mod tick_selector;
pub use tick_selector::tick_selector;
mod tick_sequence;
pub use tick_sequence::tick_sequence;

#[cfg(test)]
mod tests;

pub type ProgramCounter = Option<ExecutionPointer>;
pub type Stack = Vec<StackItem>;
pub type ReturnStack = Vec<ExecutionPointer>;
pub type Prayer = Result<Status, String>;
