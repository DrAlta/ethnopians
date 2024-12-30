use qol::logy;

use crate::sandbox::bt::{ReturnPointer, StackItem, Status};

pub fn tick_selector(
    children: &Vec<ReturnPointer>, 
    stack: &mut Vec::<StackItem>, 
    return_stack: &mut Vec::<ReturnPointer>, 
    pc: &mut Option<ReturnPointer>,
) -> Result<Status, String> {
    let Some(tos) = stack.pop() else {
        return Err("Nothing on stack when checking result of child".into())
    };

    if StackItem::Init == tos {
        stack.push(StackItem::Selector(0));
        stack.push(StackItem::Init);
        let Some(child_token) = children.first() else {
            return Err("failed to get first child".into())
        };
        logy!("trace-tick-selector", "Initalizing Selector");
        return_stack.push(child_token.clone());
        *pc = Some(child_token.clone());
        return Ok(Status::None)
    };
    logy!("trace-tick-selector", "Doing main body of Selector tick");


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
                return Ok(Status::None)
            } else {
                // the program finished
                *pc = None;
                return Ok(Status::Success)
            };
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
            stack.push(StackItem::Selector(idx + 1));
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
