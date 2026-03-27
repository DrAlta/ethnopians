use std::collections::{BTreeMap, BTreeSet};

#[derive(Debug, Clone, PartialEq)]
pub enum State {
    Selector {
        child_index: usize,
        child_state_maybe: Option<Box<Self>>,
        reason_for_failure: String,
    },
    Sequence {
        child_index: usize,
        child_state_maybe: Option<Box<Self>>,
    },
    Parallel {
        succeeded_children: BTreeSet<usize>,
        failed_children: BTreeSet<usize>,
        children_states_maybe: Option<BTreeMap<usize, Option<State>>>,
    },

    // #####
    // # Decorator
    // ###
    Inverter {
        child_state_maybe: Option<Box<Self>>,
    },

    // #####
    // # Action
    // ###
    Combine,

    // #####
    // # Condition
    // ###
    InventoryGE,
}
