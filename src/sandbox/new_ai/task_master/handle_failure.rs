use crate::sandbox::new_ai::{Status, forth::StackItem, task_master::SubSystemState};

pub fn handle_failure(stack: &mut Vec<SubSystemState>, reason: String) -> Result<(), String> {
    stack.pop();
    let Some(sub_system_state) = stack.last_mut() else {
        return Err(reason)
    };
    match sub_system_state {
        SubSystemState::BehaviorTree { returned, .. } => {
            *returned = Status::Failure{ reason };
            return Ok(());
        },
        SubSystemState::Forth{cpu, ..} => {
            cpu.stack.push(StackItem::failure(reason));
            return Ok(());
        },
    }
}
