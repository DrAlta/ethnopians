#![allow(dead_code)]

mod blackboard;
pub use blackboard::{Blackboard, Variable};
type ActionId = usize;
type ReturnPointer = usize;

#[derive(Debug)]
enum Status {
    Success,
    Failure,
    Running(ActionId),
}

enum NodeState {
    Sequence(usize),
    Selector(usize),
}


type World = (bool, bool);

