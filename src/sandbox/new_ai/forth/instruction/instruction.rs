use crate::sandbox::new_ai::{forth::{Prayer, ProgramCounter, ReturnStack, Stack, StackItem, Status, ThreadId}, InpulseId};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Instruction {
    Debug(String),
    // signels the process runing virtual machine to proform an action (? -- ?)
    // InpulseId::GoTo ( Coord -- (Success or Failure))
    // InpulseId::Take ( EntityId-- (Success or Failure))
    Action(InpulseId),
    //--------------------------------------------------------------------------
    Add,
    Call(ThreadId),
    //(Coord Coord -- Int)
    Distance,
    Div,
    Drop,
    Dup,
    FindInInventory,
    //(Coord ItemClass -- Option<ObjectId>) finds the neared item of ItemClass to ObjectId
    FindNearest,
    Eq,
    Jump(ThreadId),
    GE,
    //(BlackboardKey -- Option<_>)
    GetBlackboard,
    GetEnergy,
    //(EntityId -- Option<Int>)
    GetHP,
    //(BlackboardKey -- Option<Coord>)
    GetLocation,
    GT,
    If(usize),
    //(x -- (x false) or (Int true))
    IsInt,
    LE,
    Lit(StackItem),
    LT,
    Mul,
    NotTrue,
    Or,
    PopLast,
    Rem,
    Return,
    //(v, String -- ) puts v into the blackboard under String
    SetBlackboard,
    //(Table, v, k -- bool) stuffs v into the table under k returns true if the stuffing was successful false other wise
    Stuff,
    Sub,
    //(x -- (x false or coord true))
    SomeCoord,
    //(x -- (x false or EntityId true))
    SomeEntityId,
    //(x -- (x false or Int true))
    SomeInt,
    Swap,
    // (??? -- Succes/Failure)  Should change this to True/False but currently it uses the same prayer as the BT's InventortGE
    InventoryGE,
    IsEmpty,
    RemoveEntitiesOfType,
    RetainEntitiesOfType,
    Rot,
    // (coord coord -- Table) gets all entities in a TOS rectanle at NOS
    GetEntities,
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

