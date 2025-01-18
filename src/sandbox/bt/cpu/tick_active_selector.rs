use crate::sandbox::{bt::{Blackboard, BlackboardKey, BlackboardValue, ExecutionToken, Instruction, StackItem, Status, TreePool}, World};

use super::{ProgramCounter, ReturnStack, Stack, CPU};

/*
* ActiveSelector: normal selectors only tick one child each time they are tick. active Selector generats it's own ticks for it's children until one returns Success or running
** so instead of just setting up the CPU for the next .step() it calls .step() itself until one of the chilren Succeeds or is Running
*** it needs to check if the child is returning to itself and if so do its own return

When an ActiveSelector returns running then when executions returns it don't just resume on the child that retuned running to see if it returns Success now, instead it gows back to the first child
*/
pub fn tick_active_selector(
    child: ExecutionToken,
    stack: &mut Stack,
    return_stack: &mut ReturnStack,
    pc: &mut ProgramCounter,
    bt: &TreePool,
    blackboard: &mut Blackboard<BlackboardKey, BlackboardValue>,
    world: &World,
) -> Result<Status, String>
{
    let mut cpu = CPU::load(child);
    loop{ 
        if cpu.pc.is_none() {
            return Instruction::exit(Status::Failure, return_stack, pc)
        }
        return match cpu.step(bt, blackboard, world) {
            Ok(Status::Success) => {
                stack.push(StackItem::Success);
                Instruction::exit(Status::Success, return_stack, pc)
            },
            Ok(Status::Running(cmd)) => Ok(Status::Running(cmd)),
            Ok(_) => continue,
            Err(_) => Instruction::exit(Status::Failure, return_stack, pc),
        };
    }
}
