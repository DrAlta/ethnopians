/*
/// ActiveSeq keeps track of the last child that returned success, each time a child returns success it sets that as it's highest succes then restarts at it's first child. of a child if after the highest success fails the actice selector fails, other wares it keep evaluating it's chilren like a selector
stuct ActiveSeq{
    children: Vec<node>,
    currect: usize,
    last_success: usize,
}
impl ActiveSeq {
    fn tick(&mut self) -> Status{
    let x = self.children[self.current].tick();
    match (x, self.currect <= self.last_success) {
        (Success, _) => {
            self.last_success = self.current;
            self.current + 1;
            if self.current == self.children.len() {
                return Success
            } else {
                return Running
            }
        },
        (Failure, false) => {
            self.current = 0;
            self.last_succes = 0;
            return Failure
        },
        (Failure, true) => {
            self.current += 1;
            return Running
        },
    }
}
*/
#![allow(dead_code)]
use std::collections::HashMap;

mod blackboard;
pub use blackboard::{Blackboard, Variable};
mod blackboard_value;
pub use blackboard_value::BlackboardValue;
mod correct;
pub use correct::Corrent;
pub mod cpu;
mod instruction;
pub use instruction::Instruction;
pub mod parser;
mod stack_item;
pub use stack_item::StackItem;
mod status;
pub use status::Status;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum InpulseId {
    Act1,
    Act2,
    Act3,
}

pub type BlackboardKey = String;
pub type ThreadName = String;
pub type ExecutionToken = ThreadName;
pub type ExecutionPointer = (ExecutionToken, usize);
pub type Thread = Vec<Instruction>;
pub type TreePool = HashMap<ThreadName, Thread>;
pub type ItemId = String;
