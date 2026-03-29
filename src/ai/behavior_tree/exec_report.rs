use std::collections::BTreeMap;

use crate::ai::{behavior_tree::State, Prayer, Status};

#[derive(Debug, Clone, PartialEq)]
pub enum ExecReport<InpulseId, EntityId, Item> {
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
    Prayer(Prayer<InpulseId, EntityId, Item>),
}
