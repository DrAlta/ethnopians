use crate::sandbox::new_ai::{behavior_tree, forth, task_master::{BehaviorTreeTaskId, ForthTaskId}};

#[derive(Debug, Clone, PartialEq)]
pub struct Branch {
    pub child_index: usize,
    pub parent_up_tick_state: behavior_tree::State
}
#[derive(Debug, Clone, PartialEq)]
pub enum SubSystemState {
    BehaviorTree{
        tree_id: BehaviorTreeTaskId,
        execution_limb: Vec::<Branch>,
        state_tick_next_maybe: Option<behavior_tree::State>,
    },
    Forth{word_id: ForthTaskId, cpu: forth::CPU},
}
