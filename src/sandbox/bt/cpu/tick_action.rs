use crate::sandbox::bt::{InpulseId, ExecutionToken, StackItem, Status};

pub fn tick_action(
    action_id: &InpulseId, 
    stack: &mut Vec::<StackItem>, 
    return_stack: &mut Vec::<ExecutionToken>, 
    pc: &mut Option<ExecutionToken>,
) -> Result<Status, String> {
    let Some(tos) = stack.pop() else {
        return Err("Nothing on stack when checking result of child".into())
    };
    match tos {
        StackItem::Success => {
            stack.push(StackItem::Success);
            if let Some(parent_token) = return_stack.pop() {
                // return to calling fuction
                *pc = Some(parent_token);
                return Ok(Status::None)
            } else {
                // the program finished
                *pc = None;
                return Ok(Status::Success)
            };
        },
        StackItem::Failure => {
            stack.push(StackItem::Failure);
            if let Some(parent_token) = return_stack.pop() {
                // return to calling fuction
                *pc = Some(parent_token);
                return Ok(Status::None)
            } else {
                // the program finished
                *pc = None;
                return Ok(Status::Failure)
            };
        },
        StackItem::Init => {
            let x = if action_id == &InpulseId::Act2 {StackItem::Failure} else { StackItem::Success };
            stack.push(x);
            return Ok(Status::Running(action_id.clone()))
        },
        _ => {
            return Err("invalid Top of stack".into())
        }
    }
}
