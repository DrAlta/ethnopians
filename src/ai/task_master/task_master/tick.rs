use std::collections::HashMap;

use crate::ai::behavior_tree::{ExecReport, Node};
use crate::ai::task_master::{BehaviorTreeTaskId, SubSystemState, TaskMaster, TastMasterReport};
use crate::ai::task_master::handle_failure;
use crate::ai::task_master::task_master::{walk_down, walk_up};


impl<InpulseId: Clone, EntityId: std::hash::Hash + std::fmt::Debug + Clone, Item: Clone> TaskMaster<InpulseId, EntityId, Item> 
where for<'a> Item: TryFrom<&'a str>
{
    pub fn tick(
        &mut self, 
        behavoir_tree_tasks: &HashMap<BehaviorTreeTaskId, Node>,
        answer_to_prayer_maybe: Option<ExecReport<InpulseId, EntityId, Item>>,
    ) -> TastMasterReport<InpulseId, EntityId, Item> {
        let Some(sub_system_state) = self.stack.last_mut() else {
            self.tick_counter += 1;
            return  TastMasterReport::Err(
                format!(
                    "[{}:{}] no task to preform",
                    file!(),
                    line!()
                )
            );
        };
        match sub_system_state {
            SubSystemState::BehaviorTree{tree_id, execution_limb, state_tick_next_maybe} => {
                // this does a down_tick() then queues the next down_tick() proccessing up_ticks untill one return a require for a down_tick
                let tree_id2= tree_id.clone();
                let Some(tree) = behavoir_tree_tasks.get(tree_id) else {
                    self.tick_counter += 1;
                    match handle_failure(&mut self.stack, format!("[{}:{}] failed to get behavior tree task: {tree_id2:?}", file!(), line!())) {
                        Ok(_) => {return TastMasterReport::Ok},
                        Err(reason) => {return TastMasterReport::Failure { reason }},
                    };
                };

                let (path, this_node) = match walk_down(tree, execution_limb) {
                    Ok(x) => x,
                    Err(err) => {
                        self.tick_counter += 1;
                        return TastMasterReport::Err(err)
                    }
                };
                let exec_report = loop {
                    if let Some(answer) = answer_to_prayer_maybe {
                        if self.prayer_being_waited_on_maybe.is_some() {
                            break answer
                        } else {
                            todo!("got an anwer to a prayer but we weren't praying")
                        }
                    }
                    break this_node.down_tick(std::mem::replace(state_tick_next_maybe, None), &mut self.blackboard)
                };
                return walk_up(self.tick_counter, state_tick_next_maybe, execution_limb, exec_report, &mut self.prayer_being_waited_on_maybe, /*this_node,*/ path, tree, /*&mut self.blackboard*/);
            },
            SubSystemState::Forth{cpu:_, word_id: _} => {
                todo!()
            },
        }

    }
}
