use std::collections::HashMap;

use crate::sandbox::new_ai::{Blackboard, BlackboardKey, BlackboardValue, Prayer, Status, behavior_tree, forth::StackItem, task_master::BehaviorTreeTaskId};

use super::{SubSystemState, TastMasterRet};

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
    ) -> TastMasterRet{
        let Some(sub_system_state) = self.stack.last_mut() else {
            return  TastMasterRet::Err(
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
                    Self::handle_failure(&mut self.stack, format!("[{}:{}] failed to get behavior tree task: {tree_id2:?}", file!(), line!()));
                    return TastMasterRet::Ok;
                };

                let mut this_node = tree;
                let mut path = Vec::new();
                for (stem_idx, (child_idx, _)) in execution_path.iter().enumerate() {
                    path.push(stem_idx);
                    this_node = match this_node.get_child(*child_idx){
                        Ok(c) => c,
                        Err(err) => {
                            Self::handle_failure(&mut self.stack, format!("[{}:{}] while walking execution_path{path:?} got err{err:?}", file!(), line!()));
                            return TastMasterRet::Ok;
                        }
                    };
                }

                if let Some((this_node_idx, state)) = execution_path.pop() {
                    let x = this_node.down_tick(state, &mut self.blackboard);
                    let (my_state, child_index, child_state_maybe) = Self::handle_behavoir_tree_exec_report(x);
                    execution_path.push((this_node_idx, Some(my_state)));
                    execution_path.push((child_index, child_state_maybe));
                } else {
                    let x = this_node.down_tick(None, &mut self.blackboard);
                    let (my_state, child_index, child_state_maybe) = Self::handle_behavoir_tree_exec_report(x);
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
    pub fn handle_failure(stack: &mut Vec<SubSystemState>, reason: String) {
        stack.pop();
        let Some(sub_system_state) = stack.last_mut() else {
            return
        };
        match sub_system_state {
            SubSystemState::BehaviorTree { returned, .. } => {
                *returned = Status::Failure{ reason };
                return;
            },
            SubSystemState::Forth{cpu, ..} => {
                cpu.stack.push(StackItem::failure(reason));
                return
            },
        }
    }
    fn handle_behavoir_tree_exec_report(
//        &mut self,
        exec_report: behavior_tree::ExecReport,
    ) -> (behavior_tree::State, usize, Option<behavior_tree::State>) {
        match exec_report {
            behavior_tree::ExecReport::TickChild { child_index, my_state, child_state_maybe } => {
                (my_state, child_index, child_state_maybe)
            },
            behavior_tree::ExecReport::TickChildren { children_states } => todo!(),
            behavior_tree::ExecReport::Status { status } => todo!(),
            behavior_tree::ExecReport::Prayer(Prayer::Combine { direct_item_class, indirect_item_class }) => todo!(),
            behavior_tree::ExecReport::Prayer(Prayer::GetIsInventoryGE { agent, item_class, amount }) => todo!(),
            behavior_tree::ExecReport::Prayer(_) => todo!()
        }
    }
}