use crate::sandbox::new_ai::{forth::{ProgramCounter, ReturnStack, Stack, StackItem, Status, ThreadId, ThreadPool}, Blackboard, BlackboardKey, BlackboardValue};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CPU {
    pub pc: ProgramCounter,
    pub stack: Stack,
    pub return_stack: ReturnStack,
}

impl CPU {
    pub fn load(token: ThreadId) -> Self {
        let pc = Some((token.clone(), 0));
        let stack = vec![StackItem::init()];
        let return_stack = Vec::new();

        Self {
            pc,
            stack,
            return_stack,
        }
    }
    pub fn step(
        &mut self,
        bt: &ThreadPool,
        blackboard: &mut Blackboard<BlackboardKey, BlackboardValue>,
    ) -> Result<Status, String> {
        let Some((token, idx)) = &self.pc else {
            return Err("program halted".into());
        };

        let Some(thread) = bt.get(token) else {
            return Err("failed to get thread {token}".into());
        };
        let Some(i) = thread.get(*idx) else {
            return Err(format!(
                "failed to get instruction{idx} from thread {token}"
            ));
        };
        i.tick(
            &mut self.stack,
            &mut self.return_stack,
            &mut self.pc,
            blackboard,
        )
    }
}
