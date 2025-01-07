use std::collections::HashMap;

use super::{ExecutionToken, StackItem, Status, Thread};

#[cfg(test)]
mod tests;
mod tick_action;
pub use tick_action::tick_action;
mod tick_selector;
pub use tick_selector::tick_selector;
mod tick_sequence;
pub use tick_sequence::tick_sequence;

type ProgramCounter = Option<ExecutionToken>;
type Stack = Vec<StackItem>;
type ReturnStack = Vec<ExecutionToken>;

pub struct CPU {
    pub pc: ProgramCounter,
    pub stack: Stack,
    pub return_stack: ReturnStack
}

impl CPU {
    pub fn load(
        token: ExecutionToken,
    ) -> Self {
        let pc = Some(token.clone());
        let stack = vec![StackItem::Init];
        let return_stack = Vec::new();

        Self { pc, stack, return_stack }
    }
    pub fn step(
        &mut self,
        bt: &HashMap<ExecutionToken, Thread>,
    ) -> Result<Status, String> {
        let Some(token) = &self.pc else {
            return Err("program halted".into());
        };

        let Some(thread) = bt.get(token) else {
            return Err("failed to get thread {token}".into());
        };
        thread.tick(&mut self.stack, &mut self.return_stack, &mut self.pc)
    }
}