use std::collections::HashMap;

use crate::sandbox::{
    bt::{
        blackboard::Blackboard,
        cpu::{ProgramCounter, ReturnStack, Stack, StackItem},
        BlackboardKey, BlackboardValue, ExecutionToken, Status, Thread,
    },
    World,
};

pub struct CPU {
    pub pc: ProgramCounter,
    pub stack: Stack,
    pub return_stack: ReturnStack,
}

impl CPU {
    pub fn load(token: ExecutionToken) -> Self {
        let pc = Some((token.clone(), 0));
        let stack = vec![StackItem::Init];
        let return_stack = Vec::new();

        Self {
            pc,
            stack,
            return_stack,
        }
    }
    pub fn step(
        &mut self,
        bt: &HashMap<ExecutionToken, Thread>,
        blackboard: &mut Blackboard<BlackboardKey, BlackboardValue>,
        world: &World,
    ) -> Result<Status, String> {
        let Some((token, idx)) = &self.pc else {
            return Err("program halted".into());
        };

        let Some(thread) = bt.get(token) else {
            return Err("failed to get thread {token}".into());
        };
        let Some(i) = thread.get(*idx) else {
            return Err("failed to get instruction{idx} from thread {token}".into());
        };
        i.tick(
            &mut self.stack,
            &mut self.return_stack,
            &mut self.pc,
            blackboard,
            world,
        )
    }
}
