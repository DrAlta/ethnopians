#![allow(dead_code)]

mod blackboard;
pub use blackboard::{Blackboard, Variable};
mod node;
pub use node::Node;

type ActionId = usize;
type ReturnPointer = usize;

#[derive(Debug)]
enum Status {
    Success,
    Failure,
    Running(ActionId),
    None,
}

enum StackItem {
    //node states
    Sequence(usize),
    Selector(usize),
    // return statues
    Success,
    Failure,
    Init,
}


type World = (bool, bool);

