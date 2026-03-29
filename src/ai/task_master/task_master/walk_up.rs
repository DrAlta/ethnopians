use crate::ai::behavior_tree::{self, ExecReport, Node, State};
use crate::ai::task_master::task_master::TickCount;
use crate::ai::task_master::{sub_system_state::Branch, task_master::Path, TastMasterReport};
use crate::ai::{Prayer, Status};

pub fn walk_up<InpulseId: Clone, EntityId: std::fmt::Debug + std::hash::Hash + Clone, Item: Clone>(
    tick_counter: TickCount,
    state_tick_next_maybe: &mut Option<State>,
    execution_limb: &mut Vec<Branch>,
    mut exec_report: ExecReport<InpulseId, EntityId, Item>,
    prayer_being_waited_on_maybe: &mut Option<(Prayer<InpulseId, EntityId, Item>, TickCount)>,
    //this_node: &Node,
    mut path: Path,
    tree: &Node,
    //blackboard: &mut Blackboard<BlackboardKey, BlackboardValue<EntityId>>
) -> TastMasterReport<InpulseId, EntityId, Item>
where
    for<'a> Item: TryFrom<&'a str>,
{
    //let mut exec_report: ExecReport<InpulseId, EntityId, Item> = this_node.down_tick(std::mem::replace(state_tick_next_maybe, None), blackboard);
    loop {
        match exec_report {
            behavior_tree::ExecReport::TickChild {
                child_index,
                my_state,
                child_state_maybe,
            } => {
                *state_tick_next_maybe = child_state_maybe;
                execution_limb.push(Branch {
                    child_index,
                    parent_up_tick_state: my_state,
                });
                return TastMasterReport::Ok;
            }
            behavior_tree::ExecReport::TickChildren { children_states } => {
                todo!("{children_states:?}")
            }
            behavior_tree::ExecReport::Status { status } => {
                let Some(Branch {
                    parent_up_tick_state,
                    ..
                }) = execution_limb.pop()
                else {
                    // it must have been the root node that is upticking, report the task finished
                    return match status {
                        Status::Success => TastMasterReport::Success,
                        Status::Failure { reason } => TastMasterReport::Failure { reason },
                        Status::Waiting { state } => TastMasterReport::Err(format!(
                            "{tree:?} returned that is was waiting with {state:?}"
                        )),
                    };
                };
                path.pop(); // remove the current node from the path
                let parent_node = if let Some(parent_node) = path.last() {
                    parent_node
                } else {
                    tree
                };

                exec_report = parent_node.up_tick(parent_up_tick_state, status);
                continue;
            }
            behavior_tree::ExecReport::Prayer(prayer) => {
                *prayer_being_waited_on_maybe = Some((prayer.clone(), tick_counter));
                return TastMasterReport::Prayer(prayer);
            }
        }
        //unreachable!()
    }
}
