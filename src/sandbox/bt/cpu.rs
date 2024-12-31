use std::collections::HashMap;

use super::{Thread, ExecutionToken, StackItem, Status};

mod tick_action;
pub use tick_action::tick_action;
mod tick_selector;
pub use tick_selector::tick_selector;
mod tick_sequence;
pub use tick_sequence::tick_sequence;

pub fn load(
    token: ExecutionToken, 
    _bt: & HashMap<ExecutionToken, Thread>
) -> ( 
    Option<ExecutionToken>,
    Vec::<StackItem>, 
    Vec::<ExecutionToken>, 
) {
    let pc = Some(token.clone());
    let stack = vec![StackItem::Init];
    let return_stack = Vec::new();

    (
        pc,
        stack,
        return_stack,
    )
}
pub fn step(
    pc: &mut Option<ExecutionToken>,
    stack: &mut Vec::<StackItem>, 
    return_stack: &mut Vec::<ExecutionToken>, 
    bt: & HashMap<ExecutionToken, Thread>
) -> Result<Status, String> {
    let Some(token) = pc else {
        return Err("program halted".into())
    };

    let Some(thread) = bt.get(token) else {
        return Err("failed to get thread {token}".into())
    };
    thread.tick(stack, return_stack, pc)
}

/*
#[test]
fn test() {
    let mut bt = HashMap::<ExecutionToken, Thread>::new();
    let action1 = 0;
    bt.insert(
        action1,
        Thread::Action(1_usize.into())
    );

    let action2 = 1 ;
    bt.insert(
        action2, 
        Thread::Action(2_usize.into())
    );
    let action3 =3;
    bt.insert(
        action3,
        Thread::Action(3_usize.into())
    );

    let sequence = 4;
    bt.insert(sequence, Thread::Sequence(vec![action1, action2]));

    let selector = 5;
    bt.insert(selector, Thread::Selector(vec![sequence, action3]));

    let (mut pc, mut stack, mut rs) = load(selector, &bt);

    for _ in 0..13{
        println!("----\nStack:{stack:?}\nreturn_stack:{rs:?}");
        match step(&mut pc, &mut stack, &mut rs, &bt) {
            Ok(ok) => {
                println!("{ok:?}");
            },
            Err(err) => {
                println!("Err:{err:?}");
                break;
            }
        };
    }
    panic!("success");
}
*/

#[test]
fn step_test() {
    let mut bt = HashMap::<ExecutionToken, Thread>::new();
    let action1 = 0;
    bt.insert(
        action1,
        Thread::Action(1_usize.into())
    );

    let action2 = 1 ;
    bt.insert(
        action2, 
        Thread::Action(2_usize.into())
    );
    let action3 =3;
    bt.insert(
        action3,
        Thread::Action(3_usize.into())
    );

    let sequence = 4;
    bt.insert(sequence, Thread::Sequence(vec![action1, action2]));

    let selector = 5;
    bt.insert(selector, Thread::Selector(vec![sequence, action3]));

    let (mut pc, mut stack, mut rs) = load(selector, &bt);
    //step 1 selectpr does its init and sets the cpu up to call its first child, sequence.
    assert_eq!(
        step(&mut pc, &mut stack, &mut rs, &bt),
        Ok(Status::None)
    );
    assert_eq!(
        &stack,
        &vec![StackItem::Selector(1), StackItem::Init]
    );
    assert_eq!(
        &rs,
        &vec![selector]
    );
    //step 2 sequence intalized and set the cpu up to call its first child, action1
    assert_eq!(
        step(&mut pc, &mut stack, &mut rs, &bt),
        Ok(Status::None)
    );
    assert_eq!(
        &stack,
        &vec![StackItem::Selector(1), StackItem::Sequence(1), StackItem::Init]
    );
    assert_eq!(
        &rs,
        &vec![selector, sequence]
    );
    //step 3 action1 puts it's state on the stack,Success, and prays Running(1)
    assert_eq!(
        step(&mut pc, &mut stack, &mut rs, &bt),
        Ok(Status::Running(1))
    );
    assert_eq!(
        &stack,
        &vec![StackItem::Selector(1), StackItem::Sequence(1), StackItem::Success]
    );
    assert_eq!(
        &rs,
        &vec![selector, sequence]
    );
    assert_eq!(
        &pc,
        &Some(action1)
    );
    //step 4 action1 sees it's status on the stack and returns it seting the cpu to return to the calling thread
    assert_eq!(
        step(&mut pc, &mut stack, &mut rs, &bt),
        Ok(Status::None)
    );
    assert_eq!(
        &stack,
        &vec![StackItem::Selector(1), StackItem::Sequence(1), StackItem::Success]
    );
    assert_eq!(
        &rs,
        &vec![selector]
    );
    //step 5  sequence sets the cpu up to run it's second child, action2
    assert_eq!(
        step(&mut pc, &mut stack, &mut rs, &bt),
        Ok(Status::None)
    );
    assert_eq!(
        &stack,
        &vec![StackItem::Selector(1), StackItem::Sequence(2), StackItem::Init]
    );
    assert_eq!(
        &pc,
        &Some(action2)
    );
    assert_eq!(
        &rs,
        &vec![selector, sequence]
    );
    //step 6 action2 intalizes and prays Running(2)
    assert_eq!(
        step(&mut pc, &mut stack, &mut rs, &bt),
        Ok(Status::Running(2))
    );
    assert_eq!(
        &stack,
        &vec![StackItem::Selector(1), StackItem::Sequence(2), StackItem::Success]
    );
    assert_eq!(
        &rs,
        &vec![selector, sequence]
    );/*
    //step 
    assert_eq!(
        step(&mut pc, &mut stack, &mut rs, &bt),
        Ok(Status::None)
    );
    assert_eq!(
        &stack,
        &vec![StackItem::Selector(1), StackItem::Init]
    );
    assert_eq!(
        &rs,
        &vec![selector, sequence]
    );
    //step 
    assert_eq!(
        step(&mut pc, &mut stack, &mut rs, &bt),
        Ok(Status::None)
    );
    assert_eq!(
        &stack,
        &vec![StackItem::Selector(1), StackItem::Init]
    );
    assert_eq!(
        &rs,
        &vec![selector, sequence]
    );
    //step 
    assert_eq!(
        step(&mut pc, &mut stack, &mut rs, &bt),
        Ok(Status::None)
    );
    assert_eq!(
        &stack,
        &vec![StackItem::Selector(1), StackItem::Init]
    );
    assert_eq!(
        &rs,
        &vec![selector, sequence]
    );
    //step 
    assert_eq!(
        step(&mut pc, &mut stack, &mut rs, &bt),
        Ok(Status::None)
    );
    assert_eq!(
        &stack,
        &vec![StackItem::Selector(1), StackItem::Init]
    );
    assert_eq!(
        &rs,
        &vec![selector, sequence]
    );
    //step 
    assert_eq!(
        step(&mut pc, &mut stack, &mut rs, &bt),
        Ok(Status::None)
    );
    assert_eq!(
        &stack,
        &vec![StackItem::Selector(1), StackItem::Init]
    );
    assert_eq!(
        &rs,
        &vec![selector, sequence]
    );
    //step 
    assert_eq!(
        step(&mut pc, &mut stack, &mut rs, &bt),
        Ok(Status::None)
    );
    assert_eq!(
        &stack,
        &vec![StackItem::Selector(1), StackItem::Init]
    );
    assert_eq!(
        &rs,
        &vec![selector, sequence]
    );
    //step 
    assert_eq!(
        step(&mut pc, &mut stack, &mut rs, &bt),
        Ok(Status::None)
    );
    assert_eq!(
        &stack,
        &vec![StackItem::Selector(1), StackItem::Init]
    );
    assert_eq!(
        &rs,
        &vec![selector, sequence]
    );
    panic!("success");
    */
}


