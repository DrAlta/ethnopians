use std::collections::HashMap;

use crate::sandbox::bt::{ActionId, ReturnPointer, StackItem, Status, World};

pub enum Node {
    Sequence(Vec<ReturnPointer>),
    Selector(Vec<ReturnPointer>),
    Action(ActionId),
}
impl Node {
    fn tick_action(
        action_id: &ActionId, 
        stack: &mut Vec::<StackItem>, 
        return_stack: &mut Vec::<ReturnPointer>, 
        pc: &mut Option<ReturnPointer>,
        _world: &mut World) -> Result<Status, String> {
        let Some(tos) = stack.pop() else {
            return Err("Nothing on stack when checking result of child".into())
        };
        match tos {
            StackItem::Success => {
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
            StackItem::Init => {
                stack.push(StackItem::Success);
                return Ok(Status::Running(action_id.clone()))
            },
            _ => {
                return Err("invalid Top of stack".into())
            }
        }
    }

    fn tick_sequence(
        children: &Vec<ReturnPointer>, 
        stack: &mut Vec::<StackItem>, 
        return_stack: &mut Vec::<ReturnPointer>, 
        pc: &mut Option<ReturnPointer>,
        bt: &HashMap<ReturnPointer, Node>,
        _world: &mut World) -> Result<Status, String> {
        let Some(tos) = stack.pop() else {
            return Err("Nothing on stack when checking result of child".into())

        };
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
                } else {
                    // the program finished
                    *pc = None;
                };
                return Ok(Status::Failure)
            },
            /*
            (_, StackItem::Running(x)) => {
                //pc is pointing at us so don't need changed
                //but our state on the stack
                stack.push(StackItem::Sequence(idx));
                stack.push(StackItem::Init);
                // pray: signal to the level above that we got running
                return Ok(Status::Running(x))
            },
            */
            (true, StackItem::Success | StackItem::Init) => {
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
            (false, StackItem::Success | StackItem::Init) => {
                let child_token = children.get(idx).expect("we already check they it was within range");
                let Some(child) = bt.get(child_token) else {
                    return Err("could lookup child in TreeDB".into())
                };
                if let Err(err) = child.init(stack) {
                    return Err(format!("failed ot init child {child_token}:{err}"))
                };
                return_stack.push(child_token.clone());
                *pc = Some(child_token.clone());
                return Ok(Status::None)
            },
            (_,_) => {
                return Err("TOS wasn't a Success or a Failure".into())
            }
        }
    }
    fn init(
        &self,
        stack: &mut Vec::<StackItem>,
    ) -> Result<(), String>{
        match self {
            Node::Sequence(_vec) => {
                stack.push(StackItem::Sequence(0));
            },
            Node::Selector(_vec) => {
                stack.push(StackItem::Selector(0));
            },
            Node::Action(_action_id) => {
                stack.push(StackItem::Init);
            },
        }
        Ok(())
    }
}

pub fn load(
    _token: ReturnPointer, 
    _bt: & HashMap<ReturnPointer, Node>
) -> ( 
    Option<ReturnPointer>,
    Vec::<StackItem>, 
    Vec::<ReturnPointer>, 
) {
   todo!()
}
pub fn step(
    _pc: &mut Option<ReturnPointer>,
    _stack: &mut Vec::<StackItem>, 
    _return_stack: &mut Vec::<ReturnPointer>, 
    _bt: & HashMap<ReturnPointer, Node>
) -> Result<Status, String> {
    todo!()
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

