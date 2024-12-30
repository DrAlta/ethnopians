use qol::logy;

use crate::sandbox::bt::{ReturnPointer, StackItem, Status};

pub fn tick_sequence(
    children: &Vec<ReturnPointer>, 
    stack: &mut Vec::<StackItem>, 
    return_stack: &mut Vec::<ReturnPointer>, 
    pc: &mut Option<ReturnPointer>,
) -> Result<Status, String> {
    let Some(tos) = stack.pop() else {
        return Err("Nothing on stack when checking result of child".into())
    };

    if StackItem::Init == tos {
        stack.push(StackItem::Sequence(0));
        stack.push(StackItem::Init);
        let Some(child_token) = children.first() else {
            return Err("failed to get first child".into())
        };
        logy!("trace-tick-sequence", "Initalizing Sequence");
        return_stack.push(child_token.clone());
        *pc = Some(child_token.clone());
        return Ok(Status::None)
    };
    logy!("trace-tick-sequence", "Doing main body of Sequence tick");


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
                return Ok(Status::None)
            } else {
                // the program finished
                *pc = None;
                return Ok(Status::Failure)
            };
        },
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
