use super::{ExecutionToken, StackItem};

mod cpu;
pub use cpu::CPU;
mod tick_action;
pub use tick_action::tick_action;
mod tick_selector;
pub use tick_selector::tick_selector;
mod tick_sequence;
pub use tick_sequence::tick_sequence;

#[cfg(test)]
mod tests;

type ProgramCounter = Option<ExecutionToken>;
type Stack = Vec<StackItem>;
type ReturnStack = Vec<ExecutionToken>;
