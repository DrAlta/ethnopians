use crate::sandbox::bt::{ActionId, cpu::{tick_action, tick_selector, tick_sequence}, ReturnPointer, StackItem, Status};


pub enum Node {
    Sequence(Vec<ReturnPointer>),
    Selector(Vec<ReturnPointer>),
    Action(ActionId),
}

impl Node {
    pub fn tick(
        &self,
        stack: &mut Vec::<StackItem>, 
        return_stack: &mut Vec::<ReturnPointer>, 
        pc: &mut Option<ReturnPointer>,
    ) -> Result<Status, String> {
        match self {
            Node::Sequence(children) => tick_sequence(children, stack, return_stack, pc),
            Node::Selector(children) => tick_selector(children, stack, return_stack, pc),
            Node::Action(action_id) => tick_action(action_id, stack, return_stack, pc),
        }
    }
}