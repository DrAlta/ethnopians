use crate::sandbox::new_ai::{Prayer, behavior_tree};

pub fn handle_behavoir_tree_exec_report(
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
