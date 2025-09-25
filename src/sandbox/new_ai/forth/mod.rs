mod cpu;
pub use cpu::CPU;
mod instruction;
pub use instruction::Instruction;
mod stack_item;
pub use stack_item::{StackItem, TableGet, TableInterior};
mod status;
pub use status::Status;
mod thread_pool;
pub use thread_pool::ThreadPool;


pub type Thread = Vec<Instruction>;
pub type ThreadId = String;

type ExecutionPointer = (ThreadId, usize);

pub type ProgramCounter = Option<ExecutionPointer>;
pub type Stack = Vec<StackItem>;
pub type ReturnStack = Vec<ExecutionPointer>;
pub type Prayer = Result<Status, String>;
#[cfg(test)]
mod tests;