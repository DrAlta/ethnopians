use crate::sandbox::{
    bt::{
        cpu::{tick_action, tick_selector, tick_sequence},
        ExecutionToken, InpulseId, StackItem, Status,
    },
    ItemClass,
};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Instruction {
    Action(InpulseId),
    InventoryGE(ItemClass, u8),
    Selector(Vec<ExecutionToken>),
    Sequence(Vec<ExecutionToken>),
}

impl Instruction {
    pub fn tick(
        &self,
        stack: &mut Vec<StackItem>,
        return_stack: &mut Vec<ExecutionToken>,
        pc: &mut Option<ExecutionToken>,
    ) -> Result<Status, String> {
        match self {
            Instruction::Action(action_id) => tick_action(action_id, stack, return_stack, pc),
            Instruction::InventoryGE(_, _) => todo!(),
            Instruction::Selector(children) => tick_selector(children, stack, return_stack, pc),
            Instruction::Sequence(children) => tick_sequence(children, stack, return_stack, pc),
        }
    }
    pub fn correct(&mut self, prefix: &str) {
        match self {
            Instruction::Action(_inpulse_id) => (),
            Instruction::InventoryGE(_, _) => (),
            Instruction::Selector(vec) => {
                vec.into_iter().for_each(|x| {
                    let y = format!("{prefix}{x}");
                    *x = y
                });
            }
            Instruction::Sequence(vec) => {
                vec.into_iter().for_each(|x| {
                    let y = format!("{prefix}{x}");
                    *x = y
                });
            }
        }
    }
}
