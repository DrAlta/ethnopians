use qol::logy;

use crate::sandbox::ai::{ExecutionToken, StackItem, Status};

use super::{ProgramCounter, ReturnStack, Stack};

pub fn tick_sequence(
    children: &Vec<ExecutionToken>,
    stack: &mut Stack,
    return_stack: &mut ReturnStack,
    pc: &mut ProgramCounter,
) -> Result<Status, String> {
    let Some(tos) = stack.pop() else {
        return Err("Nothing on stack when checking result of child".into());
    };

    if if let StackItem::String(x)/*Init*/ = tos {x == "Init"} else {false} {
        // this runs the first child
        stack.push(StackItem::Sequence(1));
        stack.push(StackItem::Init);
        let Some(child_token) = children.first() else {
            return Err("failed to get first child".into());
        };
        logy!("trace-tick-sequence", "Initalizing Sequence");
        return_stack.push(pc.clone().unwrap());
        *pc = Some((child_token.clone(), 0));
        return Ok(Status::None);
        /* will setup the process so that the next execution step We'll process the first child
        stack.push(StackItem::Sequence(0));
        stack.push(StackItem::Init);
        return Ok(Status::None)
        */
    };
    logy!("trace-tick-sequence", "Doing main body of Sequence tick");

    let Some(StackItem::Sequence(idx)) = stack.pop() else {
        return Err("Sequence state not found on stack".into());
    };
    match (idx >= children.len(), tos) {
        (_, StackItem::Failure) => {
            stack.push(StackItem::Failure);
            if let Some(parent_token) = return_stack.pop() {
                // return to calling fuction
                *pc = Some(parent_token);
                return Ok(Status::None);
            } else {
                // the program finished
                *pc = None;
                return Ok(Status::Failure);
            };
        }
        (true, StackItem::Success) => {
            stack.push(StackItem::Success);
            if let Some(parent_token) = return_stack.pop() {
                // return to calling fuction
                *pc = Some(parent_token);
                return Ok(Status::None);
            } else {
                // the program finished
                *pc = None;
                return Ok(Status::Success);
            };
        }
        (false, StackItem::Success) => {
            let child_token = children
                .get(idx)
                .expect("we already check they it was within range");
            stack.push(StackItem::Sequence(idx + 1));
            stack.push(StackItem::Init);
            return_stack.push(pc.clone().unwrap());
            *pc = Some((child_token.clone(), 0));
            return Ok(Status::None);
        }
        (_, _) => return Err("TOS wasn't a Success or a Failure".into()),
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn sequence_init_test() {
        let mut stack = vec![StackItem::Init];
        let mut rs = Vec::new();
        let mut pc = Some(("1".to_owned(), 0));

        let children = vec!["42".to_owned()];

        assert_eq!(
            tick_sequence(&children, &mut stack, &mut rs, &mut pc),
            Ok(Status::None)
        );
        assert_eq!(stack, vec![StackItem::Sequence(1), StackItem::Init]);
        assert_eq!(rs, vec![("1".to_owned(), 0)]);
        assert_eq!(pc, Some(("42".to_owned(), 0)));
    }
    #[test]
    pub fn sequence_step_test() {
        let mut stack = vec![StackItem::Sequence(0), StackItem::Success];
        let mut rs = Vec::new();
        let mut pc = Some(("1".to_owned(), 0));

        let children = vec!["42".to_owned(), "69".to_owned()];

        assert_eq!(
            tick_sequence(&children, &mut stack, &mut rs, &mut pc),
            Ok(Status::None)
        );
        assert_eq!(stack, vec![StackItem::Sequence(1), StackItem::Init]);
        assert_eq!(rs, vec![("1".to_owned(), 0)]);
        assert_eq!(pc, Some(("42".to_owned(), 0)));
    }
    #[test]
    pub fn sequence_success_test() {
        let mut stack = vec![StackItem::Sequence(2), StackItem::Success];
        let mut rs = Vec::new();
        let mut pc = Some(("1".to_owned(), 0));

        let children = vec!["42".to_owned()];

        assert_eq!(
            tick_sequence(&children, &mut stack, &mut rs, &mut pc),
            Ok(Status::Success)
        );
        assert_eq!(stack, vec![StackItem::Success]);
        assert_eq!(rs, ReturnStack::new());
        assert_eq!(pc, None);
    }
    #[test]
    pub fn sequence_fail_test() {
        let mut stack = vec![StackItem::Sequence(0), StackItem::Failure];
        let mut rs = Vec::new();
        let mut pc = Some(("1".to_owned(), 0));

        let children = vec!["42".to_owned()];

        assert_eq!(
            tick_sequence(&children, &mut stack, &mut rs, &mut pc),
            Ok(Status::Failure)
        );
        assert_eq!(stack, vec![StackItem::Failure]);
        assert_eq!(rs, ReturnStack::new());
        assert_eq!(pc, None);
    }
}
