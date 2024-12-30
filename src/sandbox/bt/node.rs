use crate::sandbox::bt::{ActionId, ReturnPointer, StackItem, Status};

pub enum Node {
    Sequence(Vec<ReturnPointer>),
    Selector(Vec<ReturnPointer>),
    Action(ActionId),
}

impl Node {
    fn tick_action(
        action_id: &ActionId, 
        stack: &mut Vec::<StackItem>, 
        return_stack: &mut Vec::<ReturnPointer>, 
        pc: &mut Option<ReturnPointer>,
    ) -> Result<Status, String> {
        let Some(tos) = stack.pop() else {
            return Err("Nothing on stack when checking result of child".into())
        };
        match tos {
            StackItem::Success => {
                stack.push(StackItem::Success);
                // remove ourselve from the return stack
                return_stack.pop();
                if let Some(parent_token) = return_stack.last() {
                    // return to calling fuction
                    *pc = Some(parent_token.clone());
                } else {
                    // the program finished
                    *pc = None;
                };
                return Ok(Status::Success)
            },
            StackItem::Init => {
                stack.push(StackItem::Success);
                return Ok(Status::Running(action_id.clone()))
            },
            _ => {
                return Err("invalid Top of stack".into())
            }
        }
    }


    fn tick_sequence(
        children: &Vec<ReturnPointer>, 
        stack: &mut Vec::<StackItem>, 
        return_stack: &mut Vec::<ReturnPointer>, 
        pc: &mut Option<ReturnPointer>,
    ) -> Result<Status, String> {
        let Some(tos) = stack.pop() else {
            return Err("Nothing on stack when checking result of child".into())
        };

        let StackItem::Init = tos else {
            stack.push(StackItem::Sequence(0));
            let Some(child_token) = children.first() else {
                return Err("failed to get first child".into())
            };
            return_stack.push(child_token.clone());
            *pc = Some(child_token.clone());
            return Ok(Status::None)
        };


        let Some(StackItem::Sequence(idx)) = stack.pop() else {
            return Err("Sequence state not found on stack".into())
        };
        match (idx >= children.len(), tos) {
            (_, StackItem::Failure) => {
                stack.push(StackItem::Failure);
                return_stack.pop();
                if let Some(parent_token) = return_stack.last() {
                    // return to calling fuction
                    *pc = Some(parent_token.clone());
                } else {
                    // the program finished
                    *pc = None;
                };
                return Ok(Status::Failure)
            },
            /*
            (_, StackItem::Running(x)) => {
                //pc is pointing at us so don't need changed
                //but our state on the stack
                stack.push(StackItem::Sequence(idx));
                stack.push(StackItem::Init);
                // pray: signal to the level above that we got running
                return Ok(Status::Running(x))
            },
            */
            (true, StackItem::Success) => {
                stack.push(StackItem::Success);
                // remove ourselve from the return stack
                return_stack.pop();
                if let Some(parent_token) = return_stack.last() {
                    // return to calling fuction
                    *pc = Some(parent_token.clone());
                } else {
                    // the program finished
                    *pc = None;
                };
                return Ok(Status::Success)
            },
            (false, StackItem::Success) => {
                let child_token = children.get(idx).expect("we already check they it was within range");
                stack.push(StackItem::Init);
                return_stack.push(child_token.clone());
                *pc = Some(child_token.clone());
                return Ok(Status::None)
            },
            (_,_) => {
                return Err("TOS wasn't a Success or a Failure".into())
            }
        }
    }
    fn tick_selector(
        children: &Vec<ReturnPointer>, 
        stack: &mut Vec::<StackItem>, 
        return_stack: &mut Vec::<ReturnPointer>, 
        pc: &mut Option<ReturnPointer>,
    ) -> Result<Status, String> {
        let Some(tos) = stack.pop() else {
            return Err("Nothing on stack when checking result of child".into())
        };

        let StackItem::Init = tos else {
            stack.push(StackItem::Selector(0));
            let Some(child_token) = children.first() else {
                return Err("failed to get first child".into())
            };
            return_stack.push(child_token.clone());
            *pc = Some(child_token.clone());
            return Ok(Status::None)
        };


        let Some(StackItem::Selector(idx)) = stack.pop() else {
            return Err("Selector state not found on stack".into())
        };
        match (idx >= children.len(), tos) {
            // if we had a success then we succeed
            (_, StackItem::Success) => {
                stack.push(StackItem::Success);
                // remove ourselve from the return stack
                return_stack.pop();
                if let Some(parent_token) = return_stack.last() {
                    // return to calling fuction
                    *pc = Some(parent_token.clone());
                } else {
                    // the program finished
                    *pc = None;
                };
                return Ok(Status::Success)
            },
            (true, StackItem::Failure) => {
            // if we reached the end without a Success then we fail
                stack.push(StackItem::Failure);
                return_stack.pop();
                if let Some(parent_token) = return_stack.last() {
                    // return to calling fuction
                    *pc = Some(parent_token.clone());
                } else {
                    // the program finished
                    *pc = None;
                };
                return Ok(Status::Failure)
            },
            // if we haven't reached the end and received a Failure then try the next child
            (false, StackItem::Failure) => {
                let child_token = children.get(idx).expect("we already check they it was within range");
                stack.push(StackItem::Init);
                return_stack.push(child_token.clone());
                *pc = Some(child_token.clone());
                return Ok(Status::None)
            },
            (_,_) => {
                return Err("TOS wasn't a Success or a Failure".into())
            }
        }
    }
    pub fn tick(
        &self,
        stack: &mut Vec::<StackItem>, 
        return_stack: &mut Vec::<ReturnPointer>, 
        pc: &mut Option<ReturnPointer>,
    ) -> Result<Status, String> {
        match self {
            Node::Sequence(children) => Self::tick_sequence(children, stack, return_stack, pc),
            Node::Selector(children) => Self::tick_selector(children, stack, return_stack, pc),
            Node::Action(action_id) => Self::tick_action(action_id, stack, return_stack, pc),
        }
    }
}