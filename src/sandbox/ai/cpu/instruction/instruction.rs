use crate::sandbox::ai::{
    cpu::{Prayer, ProgramCounter, ReturnStack, Stack},
    BlackboardKey, ExecutionToken, InpulseId, StackItem, Status, ThreadName,
};

///
/// ForthFindNearest{entity_id: ObjectId, item_class: ItemClass},
/// ForthGetHP(BlackboardKey),
/// and ForthGetEnergy(BlackboardKey),
/// should probably take their argumants off the stack
///
/// should Combine, Use, Eat take a BlackboardKey that points to a ItemClass or the ItemClass directly? ether way InventoryGE should probably do the same
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Instruction {
    // signels the process runing virtual machine to proform an action (? -- ?)
    // InpulseId::GoTo ( Coord -- (Success or Failure))
    // InpulseId::Take ( EntityId-- (Success or Failure))
    Action(InpulseId),
    // takes two Blackboard keys that points to ItemClass
    Combine(BlackboardKey, BlackboardKey),
    // takes a Blackboard key that points to an ItemClass
    Eat(BlackboardKey),
    // takes a Blackboard key that points to an ItemClass and u8 of the number to compare to
    InventoryGE(BlackboardKey, i32),
    Selector(Vec<ExecutionToken>),
    Sequence(Vec<ExecutionToken>),
    // takes two Blackboard keys that points to ItemClass
    Use(BlackboardKey, BlackboardKey),
    ForthTree(ExecutionToken),
    //--------------------------------------------------------------------------
    ForthAdd,
    ForthCall(ThreadName, usize),
    //(Coord Coord -- Int)
    ForthDistance,
    ForthDiv,
    ForthDrop,
    ForthDup,
    ForthFindInInventory,
    //(Coord ItemClass -- Option<ObjectId>) finds the neared item of ItemClass to ObjectId
    ForthFindNearest,
    ForthEq,
    ForthJump(ThreadName, usize),
    ForthGE,
    //(BlackboardKey -- Option<_>)
    ForthGetBlackboard,
    ForthGetEnergy,
    //(BlackboardKey -- Option<Int>)
    ForthGetHP,
    //(BlackboardKey -- Option<Coord>)
    ForthGetLocation,
    ForthGT,
    ForthIf(usize),
    //(_ -- (_ false or Int true))
    ForthIsInt,
    ForthLE,
    ForthLit(StackItem),
    ForthLT,
    ForthMul,
    ForthNotTrue,
    ForthOr,
    ForthPopLast,
    ForthRem,
    ForthReturn,
    //(v, String) puts v into the blackboard under String
    ForthSetBlackboard,
    //(Table, v, k -- bool) stuffs v into the table under k returns true if the stuffing was successful false other wise
    ForthStuff,
    ForthSub,
    //(x -- (x false or coord true))
    ForthSomeCoord,
    //(x -- (x false or EntityId true))
    ForthSomeEntityId,
    //(x -- (x false or Int true))
    ForthSomeInt,
    ForthSwap,
    ForthInventoryGE,
    ForthIsEmpty,
    ForthRemoveEntitiesOfType,
    ForthRetainEntitiesOfType,
    ForthRot,
    // (coord coord -- Table) gets all entities in a TOS rectanle at NOS
    ForthGetEntities,
}

impl Instruction {
    pub fn next(status: Status, pc: &mut ProgramCounter) -> Prayer {
        if let Some((_, idx)) = pc {
            *idx += 1;
        }
        return Ok(status);
    }
    pub fn exit(status: Status, return_stack: &mut ReturnStack, pc: &mut ProgramCounter) -> Prayer {
        if let Some(parent_token) = return_stack.pop() {
            // return to calling fuction
            *pc = Some(parent_token);
            return Ok(status);
        } else {
            // the program finished
            *pc = None;
            return Ok(status);
        };
    }
    pub fn get_two_coords(stack: &mut Stack) -> Result<((i32, i32), (i32, i32)), String> {
        let Some(StackItem::Coord { .. }) = stack.last() else {
            return Err("top of stack not a number".into());
        };
        let Some(StackItem::Coord { .. }) = stack.get(stack.len() - 2) else {
            return Err("next of stack not a number".into());
        };
        let Some(StackItem::Coord { x: tos_x, y: tos_y }) = stack.pop() else {
            unreachable!()
        };
        let Some(StackItem::Coord { x: nos_x, y: nos_y }) = stack.pop() else {
            unreachable!()
        };
        Ok(((nos_x, nos_y), (tos_x, tos_y)))
    }
    pub fn get_two_ints(stack: &mut Stack) -> Result<(i32, i32), String> {
        let Some(StackItem::Int(_)) = stack.last() else {
            return Err("top of stack not a number".into());
        };
        let Some(StackItem::Int(_)) = stack.get(stack.len() - 2) else {
            return Err("next of stack not a number".into());
        };
        let Some(StackItem::Int(tos)) = stack.pop() else {
            unreachable!()
        };
        let Some(StackItem::Int(nos)) = stack.pop() else {
            unreachable!()
        };
        Ok((nos, tos))
    }
}

#[test]
fn correct_test() {
    let mut i = Instruction::Selector(vec!["@2".to_owned(), "@3".to_owned()]);
    i.correct("prefix");
    assert_eq!(
        i,
        Instruction::Selector(vec!["prefix@2".to_owned(), "prefix@3".to_owned()])
    )
}
