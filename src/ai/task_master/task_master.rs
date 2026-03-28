use std::collections::HashMap;

use crate::ai::{Status, Blackboard, BlackboardKey, BlackboardValue, Prayer, behavior_tree::{self, ExecReport, Node, State}, task_master::{BehaviorTreeTaskId, sub_system_state::Branch}};

use super::{SubSystemState, TastMasterReport, handle_failure};

type TickCount = u8;

#[derive(Debug, Clone, PartialEq)]
pub struct TaskMaster<InpulseId, EntityId: std::hash::Hash + std::fmt::Debug, Item> {
    blackboard: Blackboard<BlackboardKey, BlackboardValue<EntityId>>,
    stack: Vec<SubSystemState<EntityId>>,
    prayer_being_waited_on_maybe: Option<(Prayer<InpulseId, EntityId, Item>, TickCount)>
}
impl<InpulseId, EntityId: std::hash::Hash + std::fmt::Debug + Clone, Item> TaskMaster<InpulseId, EntityId, Item> 
where for<'a> Item: TryFrom<&'a str>
{
    pub fn tick(
        &mut self, 
        behavoir_tree_tasks: &HashMap<BehaviorTreeTaskId, behavior_tree::Node>,
    ) -> TastMasterReport<InpulseId, EntityId, Item> {
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
            SubSystemState::BehaviorTree{tree_id, execution_limb, state_tick_next_maybe} => {
                // this does a down_tick() then queues the next down_tick() proccessing up_ticks untill one return a require for a down_tick
                let tree_id2= tree_id.clone();
                let Some(tree) = behavoir_tree_tasks.get(tree_id) else {
                    match handle_failure(&mut self.stack, format!("[{}:{}] failed to get behavior tree task: {tree_id2:?}", file!(), line!())) {
                        Ok(_) => {return TastMasterReport::Ok},
                        Err(reason) => {return TastMasterReport::Failure { reason }},
                    };
                };

                let (path, this_node) = match walk_down(tree, execution_limb) {
                    Ok(x) => x,
                    Err(err) => return TastMasterReport::Err(err)
                };
                let exec_report = this_node.down_tick(std::mem::replace(state_tick_next_maybe, None), &mut self.blackboard);
                return walk_up(state_tick_next_maybe, execution_limb, exec_report, this_node, path, tree, &mut self.blackboard);
            },
            SubSystemState::Forth{cpu, word_id: _} => {
                todo!()
            },
        }

    }
}

type Path<'a>  = Vec<&'a Node>;
fn walk_down<'a, 'b>(tree: &'a Node, execution_limb: &'b Vec::<Branch>) -> Result<(Path<'a>, &'a Node), String>{
let mut path = Vec::new();
    let mut this_node = tree;
    for Branch{ child_index, .. } in execution_limb {
        path.push(this_node);
        this_node = match this_node.get_child(*child_index){
            Ok(c) => c,
            Err(err) => {
                return Err(format!("[{}:{}] while walking execution_limb{tree:?}:{path:?} got err{err:?}", file!(), line!()))
            }
        };
    }
    Ok((path, this_node))
}
fn walk_up<InpulseId, EntityId: std::fmt::Debug + std::hash::Hash + Clone, Item>(
    state_tick_next_maybe: & mut Option<State>,
    execution_limb: &mut Vec::<Branch>,
    mut exec_report: ExecReport<InpulseId, EntityId, Item>,
    this_node: &Node,
    mut path: Path,
    tree: &Node, 
    blackboard: &mut Blackboard<BlackboardKey, BlackboardValue<EntityId>>
) -> TastMasterReport<InpulseId, EntityId, Item>
where for<'a> Item: TryFrom<&'a str>
{
    let mut exec_report: ExecReport<InpulseId, EntityId, Item> = this_node.down_tick(std::mem::replace(state_tick_next_maybe, None), blackboard);
    loop {
        match exec_report {
            behavior_tree::ExecReport::TickChild { child_index, my_state, child_state_maybe } => {
                *state_tick_next_maybe = child_state_maybe;
                execution_limb.push(Branch { child_index, parent_up_tick_state: my_state });
                return TastMasterReport::Ok;
            },
            behavior_tree::ExecReport::TickChildren { children_states } => todo!("{children_states:?}"),
            behavior_tree::ExecReport::Status { status } => {
                let Some(Branch { parent_up_tick_state, .. }) = execution_limb.pop() else {
                    // it must have been the root node that is upticking, report the task finished
                    return match status {
                        Status::Success => TastMasterReport::Success,
                        Status::Failure { reason } => TastMasterReport::Failure { reason },
                        Status::Waiting { state } => TastMasterReport::Err(format!("{tree:?} returned that is was waiting with {state:?}")),
                    }
                };
                path.pop(); // remove the current node from the path
                let parent_node = if let Some(parent_node) = path.last() {
                    parent_node
                } else {
                    tree
                };

                exec_report = parent_node.up_tick(parent_up_tick_state, status);
                continue
            },
            behavior_tree::ExecReport::Prayer(prayer) => todo!(),
        }
        unreachable!()
    }

}