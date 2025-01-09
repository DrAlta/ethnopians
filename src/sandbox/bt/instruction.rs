use std::collections::BTreeSet;

use crate::sandbox::{
    bt::{
        cpu::{tick_action, tick_selector, tick_sequence, ProgramCounter, ReturnStack, Stack},
        ExecutionToken, InpulseId, ItemId, Status,
    },
    ItemClass,
};

use super::TreePool;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Instruction {
    Action(InpulseId),
    Combine(ItemId, ItemId),
    Eat(ItemId),
    InventoryGE(ItemClass, u8),
    Selector(Vec<ExecutionToken>),
    Sequence(Vec<ExecutionToken>),
    Use(ItemId, ItemId),
    //
    /*
    ForthGetHP,
    ForthLit(StackItem),
    ForthAdd,
    ForthSub,
    ForthMul,
    ForthDiv,
    ForthRem,
    ForthGT,
    ForthLT,
    ForthGE,
    ForthLE,
    */
}

impl Instruction {
    pub fn missing_threads_used(&self, bt: &TreePool) -> BTreeSet<ExecutionToken> {
        let mut missing = BTreeSet::new();
        match self {
            Instruction::Action(_inpulse_id) => (),
            Instruction::Combine(_, _) => (),
            Instruction::Eat(_) => (),
            Instruction::InventoryGE(_, _) => (),
            Instruction::Selector(vec) | Instruction::Sequence(vec) => {
                for token in vec {
                    if !bt.contains_key(token) {
                        missing.insert(token.clone());
                    }
                }
            }
            Instruction::Use(_, _) => (),
        }
        missing
    }
    pub fn tick(
        &self,
        stack: &mut Stack,
        return_stack: &mut ReturnStack,
        pc: &mut ProgramCounter,
    ) -> Result<Status, String> {
        match self {
            Instruction::Action(action_id) => tick_action(action_id, stack, return_stack, pc),
            Instruction::Combine(_, _) => todo!(),
            Instruction::Eat(_) => todo!(),
            Instruction::InventoryGE(_, _) => todo!(),
            Instruction::Selector(children) => tick_selector(children, stack, return_stack, pc),
            Instruction::Sequence(children) => tick_sequence(children, stack, return_stack, pc),
            Instruction::Use(_, _) => todo!(),
        }
    }
    pub fn correct(&mut self, prefix: &str) {
        match self {
            Instruction::Action(_inpulse_id) => (),
            Instruction::Combine(_, _) => (),
            Instruction::Eat(_) => (),
            Instruction::InventoryGE(_, _) => (),
            Instruction::Selector(vec) | Instruction::Sequence(vec) => {
                vec.into_iter().for_each(|x| {
                    if x.starts_with('_') {
                        let y = format!("{prefix}{x}");
                        *x = y
                    };
                });
            }
            Instruction::Use(_, _) => (),
        }
    }
}

#[test]
fn correct_test() {
    let mut i = Instruction::Selector(vec!["_2".to_owned(), "_3".to_owned()]);
    i.correct("prefix");
    assert_eq!(
        i,
        Instruction::Selector(vec!["prefix_2".to_owned(), "prefix_3".to_owned()])
    )
}
