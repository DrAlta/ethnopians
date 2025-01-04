use qol::logy;

use crate::sandbox::bt::{ExecutionToken, StackItem, Status};

pub fn tick_selector(
    children: &Vec<ExecutionToken>,
    stack: &mut Vec<StackItem>,
    return_stack: &mut Vec<ExecutionToken>,
    pc: &mut Option<ExecutionToken>,
) -> Result<Status, String> {
    let Some(tos) = stack.pop() else {
        return Err("Nothing on stack when checking result of child".into());
    };

    if StackItem::Init == tos {
        // this goes directly to the fist child
        stack.push(StackItem::Selector(1));
        stack.push(StackItem::Init);
        let Some(child_token) = children.first() else {
            return Err("failed to get first child".into());
        };
        logy!("trace-tick-selector", "Initalizing Selector");
        return_stack.push(pc.clone().unwrap());
        *pc = Some(child_token.clone());
        return Ok(Status::None);

        /* this just initalize and lets the next stepping of the execution handle it
        stack.push(StackItem::Selector(0));
        stack.push(StackItem::Init);
        return Ok(Status::None)
        */
    };
    logy!("trace-tick-selector", "Doing main body of Selector tick");

    let Some(StackItem::Selector(idx)) = stack.pop() else {
        return Err("Selector state not found on stack".into());
    };
    match (idx >= children.len(), tos) {
        // if we had a success then we succeed
        (_, StackItem::Success) => {
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
        (true, StackItem::Failure) => {
            // if we reached the end without a Success then we fail
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
        // if we haven't reached the end and received a Failure then try the next child
        (false, StackItem::Failure) => {
            let child_token = children
                .get(idx)
                .expect("we already check they it was within range");
            stack.push(StackItem::Selector(idx + 1));
            stack.push(StackItem::Init);
            return_stack.push(pc.clone().unwrap());
            *pc = Some(child_token.clone());
            return Ok(Status::None);
        }
        (_, _) => return Err("TOS wasn't a Success or a Failure".into()),
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn selector_init_test() {
        let mut stack = vec![StackItem::Init];
        let mut rs = Vec::new();
        let mut pc = Some("1".to_owned());

        let children = vec!["42".to_owned()];

        assert_eq!(
            tick_selector(&children, &mut stack, &mut rs, &mut pc),
            Ok(Status::None)
        );
        assert_eq!(stack, vec![StackItem::Selector(1), StackItem::Init]);
        assert_eq!(rs, vec!["1".to_owned()]);
        assert_eq!(pc, Some("42".to_owned()));
    }
    /*
        #[test]
        pub fn sequence_step_test() {
            let mut stack = vec![StackItem::Sequence(0), StackItem::Success];
            let mut rs = vec![1];
            let mut pc = Some(1);

            let children = vec![42, 69];

            assert_eq!(
                tick_sequence(&children, &mut stack, &mut rs, &mut pc),
                Ok(Status::None)
            );
            assert_eq!(
                stack,
                vec![StackItem::Sequence(1), StackItem::Init]
            );
            assert_eq!(
                rs,
                vec![1, 42]
            );
            assert_eq!(
                pc,
                Some(42)
            );
        }
        #[test]
        pub fn sequence_success_test() {
            let mut stack = vec![StackItem::Sequence(2), StackItem::Success];
            let mut rs = vec![1];
            let mut pc = Some(1);

            let children = vec![42];

            assert_eq!(
                tick_sequence(&children, &mut stack, &mut rs, &mut pc),
                Ok(Status::Success)
            );
            assert_eq!(
                stack,
                vec![StackItem::Success]
            );
            assert_eq!(
                rs,
                vec![]
            );
            assert_eq!(
                pc,
                None
            );
        }
        #[test]
        pub fn sequence_fail_test() {
            let mut stack = vec![StackItem::Sequence(0), StackItem::Failure];
            let mut rs = vec![1];
            let mut pc = Some(1);

            let children = vec![42];

            assert_eq!(
                tick_sequence(&children, &mut stack, &mut rs, &mut pc),
                Ok(Status::Failure)
            );
            assert_eq!(
                stack,
                vec![StackItem::Failure]
            );
            assert_eq!(
                rs,
                vec![]
            );
            assert_eq!(
                pc,
                None
            );
        }
    */
}
