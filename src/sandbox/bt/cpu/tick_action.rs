use crate::sandbox::bt::{ActionId, ReturnPointer, StackItem, Status};

pub fn tick_action(
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
                return Ok(Status::None)
            } else {
                // the program finished
                *pc = None;
                return Ok(Status::Success)
            };
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
