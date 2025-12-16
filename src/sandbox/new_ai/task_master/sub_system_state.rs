use crate::sandbox::new_ai::{Status, behavior_tree, forth, task_master::{BehaviorTreeTaskId, ForthTaskId}};

#[derive(Debug, Clone, PartialEq)]
pub enum SubSystemState {
    BehaviorTree{
        tree_id: BehaviorTreeTaskId,
        root_state: behavior_tree::State,
        execution_path: Vec::<(usize, Option<behavior_tree::State>)>,
        returned: Status<behavior_tree::State>,
    },
    Forth{word_id: ForthTaskId, cpu: forth::CPU},
}
