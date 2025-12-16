mod cpu;
pub use cpu::CPU;
mod instruction;
pub use instruction::Instruction;
/*
mod prayer;
pub use prayer::Prayer;
*/
mod stack_item;
pub use stack_item::{StackItem, TableGet, TableInterior};
mod thread_pool;
pub use thread_pool::ThreadPool;

pub type Thread = Vec<Instruction>;
pub type ThreadId = String;

type ExecutionPointer = (ThreadId, usize);

pub type ProgramCounter = Option<ExecutionPointer>;
pub type Stack = Vec<StackItem>;
pub type ReturnStack = Vec<ExecutionPointer>;

#[cfg(test)]
mod tests;
