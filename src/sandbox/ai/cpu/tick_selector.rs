use qol::logy;

use crate::sandbox::ai::{
    stack_item::{TableGet, TableInterior}, ExecutionToken, Instruction, StackItem, Status,
};

use super::{ProgramCounter, ReturnStack, Stack};

pub fn tick_selector(
    children: &Vec<ExecutionToken>,
    stack: &mut Stack,
    return_stack: &mut ReturnStack,
    pc: &mut ProgramCounter,
) -> Result<Status, String> {
    logy!("trace-tick-selector", "executing Selector:{stack:?}");
    logy!("trace-tick-selector", "{return_stack:?}");

    let Some(tos) = stack.pop() else {
        return Err("Nothing on stack when checking result of child".into());
    };

    if StackItem::init() == tos {
        // this goes directly to the fist child
        stack.push(
            //    StackItem::Selector(1)
            StackItem::selector(1),
        );
        stack.push(StackItem::init());
        let Some(child_token) = children.first() else {
            return Err("failed to get first child".into());
        };
        logy!("trace-tick-selector", "Initalizing Selector");
        return_stack.push(pc.clone().unwrap());
        *pc = Some((child_token.clone(), 0));
        return Ok(Status::None);

        /* this just initalize and lets the next stepping of the execution handle it
        stack.push(StackItem::Selector(0));
        stack.push(StackItem::Init);
        return Ok(Status::None)
        */
    };
    logy!("trace-tick-selector", "Doing main body of Selector tick");

    let Some(StackItem::Table(x)) = stack.pop() else {
        logy!("debug", "{stack:#?}");
        return Err("Selector state not found on stack".into());
    };
    let TableInterior { map, parents: _ } = x.as_ref();
    let map2 = map.borrow();
    
    let Some(StackItem::Int(idx)) = map2.table_get("Selector") else {
        logy!("debug", "{map:#?}");
        return Err("Selector state not found on stack".into());
    };

    match (*idx as usize >= children.len(), tos) {
        // if we had a success then we succeed
        (_, StackItem::String(x)) if x == "Success" => {
            stack.push(StackItem::success());
            let status = if pc.is_none() {
                Status::None
            } else {
                Status::Success
            };
            Instruction::exit(status, return_stack, pc)
        }
        (true, StackItem::String(x)) if x == "Failure" => {
            // if we reached the end without a Success then we fail
            stack.push(StackItem::failure());
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
        (false, StackItem::String(x)) if x == "Failure" => {
            let child_token: &ExecutionToken = children
                .get(*idx as usize)
                .expect("we already check they it was within range");
            stack.push(StackItem::selector(idx + 1));
            stack.push(StackItem::init());
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
    pub fn selector_init_test() {
        let mut stack = vec![StackItem::init()];
        let mut rs = Vec::new();
        let mut pc = Some(("1".to_owned(), 0));

        let children = vec!["42".to_owned()];

        assert_eq!(
            tick_selector(&children, &mut stack, &mut rs, &mut pc),
            Ok(Status::None)
        );
        assert_eq!(stack, vec![StackItem::selector(1), StackItem::init()]);
        assert_eq!(rs, vec![("1".to_owned(), 0)]);
        assert_eq!(pc, Some(("42".to_owned(), 0)));
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
