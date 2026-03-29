use crate::ai::{forth::StackItem, task_master::SubSystemState};

pub fn handle_failure<EntityId>(
    stack: &mut Vec<SubSystemState<EntityId>>,
    reason: String,
) -> Result<(), String> {
    stack.pop();
    let Some(sub_system_state) = stack.last_mut() else {
        return Err(reason);
    };
    match sub_system_state {
        SubSystemState::BehaviorTree { .. } => {
            return Ok(());
        }
        SubSystemState::Forth { cpu, .. } => {
            cpu.stack.push(StackItem::failure(reason));
            return Ok(());
        }
    }
}
