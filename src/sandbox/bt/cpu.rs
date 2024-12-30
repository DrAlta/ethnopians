use std::collections::HashMap;

use super::{Node, ReturnPointer, StackItem, Status};

pub fn load(
    token: ReturnPointer, 
    _bt: & HashMap<ReturnPointer, Node>
) -> ( 
    Option<ReturnPointer>,
    Vec::<StackItem>, 
    Vec::<ReturnPointer>, 
) {
    let pc = Some(token.clone());
    let stack = vec![StackItem::Init];
    let return_stack = vec![token];

    (
        pc,
        stack,
        return_stack,
    )
}
pub fn step(
    pc: &mut Option<ReturnPointer>,
    stack: &mut Vec::<StackItem>, 
    return_stack: &mut Vec::<ReturnPointer>, 
    bt: & HashMap<ReturnPointer, Node>
) -> Result<Status, String> {
    let Some(token) = pc else {
        return Err("program halted".into())
    };

    let Some(thread) = bt.get(token) else {
        return Err("failed to get thread {token}".into())
    };
    thread.tick(stack, return_stack, pc, bt)
}


#[test]
fn test() {
    let mut bt = HashMap::<ReturnPointer, Node>::new();
    let action1 = 0;
    bt.insert(
        action1,
        Node::Action(1_usize.into())
    );

    let action2 = 1 ;
    bt.insert(
        action2, 
        Node::Action(2_usize.into())
    );
    let action3 =3;
    bt.insert(
        action3,
        Node::Action(3_usize.into())
    );

    let sequence = 4;
    bt.insert(sequence, Node::Sequence(vec![action1, action2]));

    let selector = 5;
    bt.insert(selector, Node::Selector(vec![sequence, action3]));

    let (mut pc, mut stack, mut rs) = load(selector, &bt);

    for _ in 0..3{
        println!("----\n{rs:?}\n----");
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


