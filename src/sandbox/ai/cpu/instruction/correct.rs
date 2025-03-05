use crate::sandbox::ai::
    cpu::Instruction
;

impl Instruction {
    pub fn correct(&mut self, prefix: &str) {
        match self {
            Instruction::Selector(vec) | Instruction::Sequence(vec) => {
                vec.into_iter().for_each(|x| {
                    if x.starts_with('@') {
                        let y = format!("{prefix}{x}");
                        *x = y
                    };
                });
            }
            Instruction::ForthCall(token, ..) => {
                if token.starts_with('@') {
                    let y = format!("{prefix}{token}");
                    *token = y
                };
            }
            Instruction::ForthJump(token, ..) => {
                if token.starts_with('@') {
                    let y = format!("{prefix}{token}");
                    *token = y
                };
            }
            Instruction::ForthTree(token) => {
                if token.starts_with('@') {
                    let y = format!("{prefix}{token}");
                    *token = y
                };
            }
            Instruction::Action(_)
            | Instruction::Combine(_, _)
            | Instruction::Eat(_)
            | Instruction::InventoryGE(_, _)
            | Instruction::Use(_, _)
            | Instruction::ForthGetHP
            | Instruction::ForthGetEnergy
            | Instruction::ForthLit(_)
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
            | Instruction::ForthOr
            | Instruction::ForthSetBlackboard
            | Instruction::ForthStuff
            | Instruction::ForthRetainEntitiesOfType
            | Instruction::ForthNotTrue
            | Instruction::ForthInventoryGE
            | Instruction::ForthIf(_) => (),
        }
    }
}


