use std::{collections::HashMap, fmt::format};

use crate::sandbox::new_ai::{Blackboard, BlackboardKey, BlackboardValue, Prayer, Status, behavior_tree, forth::StackItem, task_master::BehaviorTreeTaskId};

use super::{SubSystemState, TastMasterReport, handle_failure};

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
    ) -> TastMasterReport {
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
            SubSystemState::BehaviorTree{tree_id , root_state, execution_path, returned, child_to_tick_next_maybe, state_tick_next_maybe} => {
                // this does a down_tick() then queues the next down_tick() proccessing up_ticks untill one return a require for a down_tick
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
                    path.push((this_node, stem_idx));
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
                let this_node_idx_maybe = {
                    if let Some(next_child_idx) = child_to_tick_next_maybe {
                        // there is tail to set it up for a down tick
                        this_node = match this_node.get_child(*next_child_idx){
                            Ok(c) => c,
                            Err(err) => {
                                match handle_failure(&mut self.stack, format!("[{}:{}] while walking execution_path{path:?} got err{err:?}", file!(), line!())) {
                                    Ok(_) => {return TastMasterReport::Ok},
                                    Err(reason) => {return TastMasterReport::Failure { reason }},
                                };
                            }
                        };
                        Some(*next_child_idx)
                    } else { 
                        // there was no tail end to the path 
                        path.pop();
                        if let Some((this_node_idx, _up_tick_state)) = execution_path.pop() {
                            unreachable!("We should never get here the only time there shoul't be a tail is whrn were ticking the root"); //Some(this_node_idx)
                        } else {
                            None
                        }
                     }
            
                };
                let mut exec_report = this_node.down_tick(std::mem::replace(state_tick_next_maybe, None), &mut self.blackboard);
                loop {
                    match exec_report {
                        behavior_tree::ExecReport::TickChild { child_index, my_state, child_state_maybe } => {
                            if let Some(this_node_idx) = this_node_idx_maybe {
                                execution_path.push((this_node_idx, my_state));
                            } else {
                                *root_state = my_state;
                            };
                            *state_tick_next_maybe = child_state_maybe;
                            *child_to_tick_next_maybe = Some(child_index);
                        },
                        behavior_tree::ExecReport::TickChildren { children_states } => todo!(),
                        behavior_tree::ExecReport::Status { status } => {
                            if let Some((parent_node, parent_idx)) = path.pop() {
                                let Some((parent_node_idx, parent_state)) = execution_path.pop() else {
                                    let path: Vec<usize> = path.into_iter().map(|(_, x)| x).chain([parent_idx].into_iter()).collect();
                
                                    let reason = format!("failed to pop state for {path:?} of {tree:?}");
                                    return TastMasterReport::Failure { reason }
                                };
                                exec_report = parent_node.up_tick(parent_state, status)
                            } else {
                                // returning to the root node
                                assert!(execution_path.is_empty());
                                exec_report = tree.up_tick(root_state.clone(), status)
                            }
                        },
                        behavior_tree::ExecReport::Prayer(prayer) => todo!(),
                    }
                    unreachable!()
                }
            },
            SubSystemState::Forth{cpu, word_id: _} => {
                todo!()
            },
        }

    }
}