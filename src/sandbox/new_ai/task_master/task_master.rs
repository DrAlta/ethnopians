use std::collections::HashMap;

use crate::sandbox::new_ai::{Blackboard, BlackboardKey, BlackboardValue, Prayer, Status, behavior_tree, forth::StackItem, task_master::BehaviorTreeTaskId};

use super::{SubSystemState, TastMasterReport, handle_behavoir_tree_exec_report, handle_failure};

type TickCount = u8;

#[derive(Debug, Clone, PartialEq)]
pub struct TaskMaster {
    blackboard: Blackboard<BlackboardKey, BlackboardValue>,
    stack: Vec<SubSystemState>,
    prayer_being_waited_on_maybe: Option<(Prayer, TickCount)>
}
impl TaskMaster {
    pub fn tick(
        &mut self, 
        behavoir_tree_tasks: &HashMap<BehaviorTreeTaskId, behavior_tree::Node>,
    ) -> TastMasterReport{
        let Some(sub_system_state) = self.stack.last_mut() else {
            return  TastMasterReport::Err(
                format!(
                    "[{}:{}] no task to preform",
                    file!(),
                    line!()
                )
            );
        };
        match sub_system_state {
            SubSystemState::BehaviorTree{tree_id , root_state, execution_path, returned} => {
                let tree_id2= tree_id.clone();
                let Some(tree) = behavoir_tree_tasks.get(tree_id) else {
                    match handle_failure(&mut self.stack, format!("[{}:{}] failed to get behavior tree task: {tree_id2:?}", file!(), line!())) {
                        Ok(_) => {return TastMasterReport::Ok},
                        Err(reason) => {return TastMasterReport::Failure { reason }},
                    };
                };

                let mut this_node = tree;
                let mut path = Vec::new();
                for (stem_idx, (child_idx, _)) in execution_path.iter().enumerate() {
                    path.push(stem_idx);
                    this_node = match this_node.get_child(*child_idx){
                        Ok(c) => c,
                        Err(err) => {
                            match handle_failure(&mut self.stack, format!("[{}:{}] while walking execution_path{path:?} got err{err:?}", file!(), line!())) {
                                Ok(_) => {return TastMasterReport::Ok},
                                Err(reason) => {return TastMasterReport::Failure { reason }},
                            };
                        }
                    };
                }

                if let Some((this_node_idx, state)) = execution_path.pop() {
                    let x = this_node.down_tick(state, &mut self.blackboard);
                    let (my_state, child_index, child_state_maybe) = handle_behavoir_tree_exec_report(x);
                    execution_path.push((this_node_idx, Some(my_state)));
                    execution_path.push((child_index, child_state_maybe));
                } else {
                    let x = this_node.down_tick(None, &mut self.blackboard);
                    let (my_state, child_index, child_state_maybe) = handle_behavoir_tree_exec_report(x);
                    *root_state = my_state;
                    execution_path.push((child_index, child_state_maybe));
                };


                todo!()
            },
            SubSystemState::Forth{cpu, word_id: _} => {
                todo!()
            },
        }

    }
}