use std::sync::Arc;

use crate::ai::{
    Blackboard, BlackboardKey, BlackboardValue, Prayer, forth::{ProgramCounter, ReturnStack, Stack, StackItem, ThreadId, ThreadPool}
};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CPU<EntityId> {
    pub pc: ProgramCounter,
    pub stack: Stack<EntityId>,
    pub return_stack: ReturnStack,
}

impl<EntityId> CPU<EntityId> {
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
}
impl<EntityId: std::hash::Hash + std::fmt::Debug + std::fmt::Display + Clone + std::cmp::PartialEq + Ord> CPU<EntityId> {

    pub fn step<InpulseId: std::fmt::Debug + Clone, Item: TryFrom<Arc<String>>> (
        &mut self,
        bt: &ThreadPool<InpulseId, EntityId>,
        blackboard: &mut Blackboard<BlackboardKey, BlackboardValue<EntityId>>,
    ) -> Result<Option<Prayer<InpulseId, EntityId, Item>>, String>
    where for<'a> Item: TryFrom<&'a str> {
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
