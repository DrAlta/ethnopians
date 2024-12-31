use crate::sandbox::bt::{ActionId, cpu::{tick_action, tick_selector, tick_sequence}, ExecutionToken, StackItem, Status};


pub enum Thread {
    Sequence(Vec<ExecutionToken>),
    Selector(Vec<ExecutionToken>),
    Action(ActionId),
}

impl Thread {
    pub fn tick(
        &self,
        stack: &mut Vec::<StackItem>, 
        return_stack: &mut Vec::<ExecutionToken>, 
        pc: &mut Option<ExecutionToken>,
    ) -> Result<Status, String> {
        match self {
            Thread::Sequence(children) => tick_sequence(children, stack, return_stack, pc),
            Thread::Selector(children) => tick_selector(children, stack, return_stack, pc),
            Thread::Action(action_id) => tick_action(action_id, stack, return_stack, pc),
        }
    }
}