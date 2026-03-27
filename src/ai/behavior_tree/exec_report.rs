use std::collections::BTreeMap;

use crate::sandbox::new_ai::{Prayer, Status, behavior_tree::State};

#[derive(Debug, Clone, PartialEq)]
pub enum ExecReport{
    TickChild {
        child_index: usize,
        my_state: State,
        child_state_maybe: Option<State>,
    },
    TickChildren {
        children_states: BTreeMap<usize, Option<State>>,
    },
    Status {
        status: Status<State>,
    },
    Prayer(Prayer),
}