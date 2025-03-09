use std::collections::BTreeSet;

use qol::logy;

use crate::sandbox::ai::{cpu::Instruction, ExecutionToken, TreePool};
impl Instruction {
    pub fn missing_threads_used(&self, bt: &TreePool) -> BTreeSet<ExecutionToken> {
        let mut missing = BTreeSet::new();
        match self {
            Instruction::Selector(vec) | Instruction::Sequence(vec) => {
                for token in vec {
                    if !bt.contains_key(token) {
                        missing.insert(token.clone());
                    }
                }
            }
            Instruction::ForthCall(token, _idx) => {
                if token == "remove_entities_of_type" {
                    logy!(
                        "debug",
                        "call to remove_entities_of_type was processed. contained:{}",
                        bt.contains_key(token)
                    );
                }
                if !bt.contains_key(token) {
                    missing.insert(token.clone());
                }
            }
            Instruction::ForthJump(token, _idx) => {
                if !bt.contains_key(token) {
                    missing.insert(token.clone());
                }
            }
            Instruction::ForthAction(..)
            | Instruction::Combine(_, _)
            | Instruction::Eat(_)
            | Instruction::InventoryGE(_, _)
            | Instruction::Use(_, _)
            | Instruction::ForthGetHP
            | Instruction::ForthGetEnergy
            | Instruction::ForthLit(..)
            | Instruction::ForthAdd
            | Instruction::ForthSub
            | Instruction::ForthMul
            | Instruction::ForthDiv
            | Instruction::ForthRem
            | Instruction::ForthGT
            | Instruction::ForthLT
            | Instruction::ForthGE
            | Instruction::ForthLE
            | Instruction::ForthIsInt
            | Instruction::ForthReturn
            | Instruction::ForthFindInInventory
            | Instruction::ForthFindNearest
            | Instruction::ForthGetBlackboard
            | Instruction::ForthGetLocation
            | Instruction::ForthOr
            | Instruction::ForthSomeCoord
            | Instruction::ForthSomeInt
            | Instruction::ForthSomeEntityId
            | Instruction::ForthDistance
            | Instruction::ForthDup
            | Instruction::ForthSwap
            | Instruction::ForthEq
            | Instruction::ForthDrop
            | Instruction::ForthIsEmpty
            | Instruction::ForthGetEntities
            | Instruction::ForthRemoveEntitiesOfType
            | Instruction::ForthPopLast
            | Instruction::ForthSetBlackboard
            | Instruction::ForthStuff
            | Instruction::ForthRetainEntitiesOfType
            | Instruction::ForthNotTrue
            | Instruction::ForthInventoryGE
            | Instruction::ForthRot
            | Instruction::ForthIf(_) => (),
            Instruction::ForthTree(token) => {
                if !bt.contains_key(token) {
                    missing.insert(token.clone());
                }
            }
        }
        missing
    }
}
