use crate::sandbox::new_ai::{
    Blackboard, BlackboardKey, BlackboardValue, Prayer, forth::{ProgramCounter, ReturnStack, Stack, StackItem, ThreadId, ThreadPool}
};

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
    ) -> Result<Option<Prayer>, String> {
        let Some((token, idx)) = &self.pc else {
            return Err(format!("{}:{}:program halted", file!(), line!()));
        };

        let Some(thread) = bt.get(token) else {
            return Err(format!("{}:{}:failed to get thread {token}", file!(), line!()));
        };
        let Some(i) = thread.get(*idx) else {
            return Err(format!("{}:{}:failed to get instruction{idx} from thread {token}", file!(), line!()));
        };
        i.tick(
            &mut self.stack,
            &mut self.return_stack,
            &mut self.pc,
            blackboard,
        )
    }
}
