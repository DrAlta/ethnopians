use qol::logy;

use crate::sandbox::ai::{stack_item::TableInterior, ExecutionToken, StackItem, Status};

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

    if if let StackItem::String(x)/*Init*/ = &tos {x == "Init"} else {false} {
        // this runs the first child
        stack.push(StackItem::sequence(1));
        stack.push(StackItem::init());
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

    let Some(StackItem::Table(x)) = stack.pop() else {
        logy!("debug", "{stack:#?}");
        return Err("Sequence state not found on stack".into());
    };
    let TableInterior { map, parents: _ } = x.as_ref();
    let map2 = map.borrow();
    let Some(StackItem::Int(idx)) = map2.get(&StackItem::String("Sequence".to_owned())) else {
        logy!("debug", "{map:#?}");
        return Err("Sequence state not found on stack".into());
    };

    match (*idx as usize >= children.len(), tos) {
        (_, StackItem::String(x)) if x == "Failure" => {
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
        (true, StackItem::String(x)) if x == "Success" => {
            stack.push(StackItem::success());
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
        (false, StackItem::String(x)) if x == "Success" => {
            let child_token = children
                .get(*idx as usize)
                .expect("we already check they it was within range");
            stack.push(StackItem::sequence(idx + 1));
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
    pub fn sequence_init_test() {
        let mut stack = vec![StackItem::init()];
        let mut rs = Vec::new();
        let mut pc = Some(("1".to_owned(), 0));

        let children = vec!["42".to_owned()];

        assert_eq!(
            tick_sequence(&children, &mut stack, &mut rs, &mut pc),
            Ok(Status::None)
        );
        assert_eq!(stack, vec![StackItem::sequence(1), StackItem::init()]);
        assert_eq!(rs, vec![("1".to_owned(), 0)]);
        assert_eq!(pc, Some(("42".to_owned(), 0)));
    }
    #[test]
    pub fn sequence_step_test() {
        let mut stack = vec![StackItem::sequence(0), StackItem::success()];
        let mut rs = Vec::new();
        let mut pc = Some(("1".to_owned(), 0));

        let children = vec!["42".to_owned(), "69".to_owned()];

        assert_eq!(
            tick_sequence(&children, &mut stack, &mut rs, &mut pc),
            Ok(Status::None)
        );
        assert_eq!(stack, vec![StackItem::sequence(1), StackItem::init()]);
        assert_eq!(rs, vec![("1".to_owned(), 0)]);
        assert_eq!(pc, Some(("42".to_owned(), 0)));
    }
    #[test]
    pub fn sequence_success_test() {
        let mut stack = vec![StackItem::sequence(2), StackItem::success()];
        let mut rs = Vec::new();
        let mut pc = Some(("1".to_owned(), 0));

        let children = vec!["42".to_owned()];

        assert_eq!(
            tick_sequence(&children, &mut stack, &mut rs, &mut pc),
            Ok(Status::Success)
        );
        assert_eq!(stack, vec![StackItem::success()]);
        assert_eq!(rs, ReturnStack::new());
        assert_eq!(pc, None);
    }
    #[test]
    pub fn sequence_fail_test() {
        let mut stack = vec![StackItem::sequence(0), StackItem::failure()];
        let mut rs = Vec::new();
        let mut pc = Some(("1".to_owned(), 0));

        let children = vec!["42".to_owned()];

        assert_eq!(
            tick_sequence(&children, &mut stack, &mut rs, &mut pc),
            Ok(Status::Failure)
        );
        assert_eq!(stack, vec![StackItem::failure()]);
        assert_eq!(rs, ReturnStack::new());
        assert_eq!(pc, None);
    }
}
