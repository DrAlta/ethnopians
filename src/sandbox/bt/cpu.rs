use std::collections::HashMap;

use super::{ExecutionToken, StackItem, Status, Thread};

#[cfg(test)]
mod tests;
mod tick_action;
pub use tick_action::tick_action;
mod tick_selector;
pub use tick_selector::tick_selector;
mod tick_sequence;
pub use tick_sequence::tick_sequence;

type ProgramCounter = Option<ExecutionToken>;
type Stack = Vec::<StackItem>;
type ReturnStack = Vec::<ExecutionToken>;

pub fn load(
    token: ExecutionToken, 
    _bt: &HashMap<ExecutionToken, Thread>
) -> ( 
    ProgramCounter,
    Stack, 
    ReturnStack, 
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
/* {
    use super::*;

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