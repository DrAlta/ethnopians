use std::collections::BTreeMap;

use crate::sandbox::new_ai::behavior_tree::{Node, Prayer, State, Status};

impl Node {
    pub fn multi_up_tick(
        &self,
        state: State,
        childerns_returned_statuses: BTreeMap<usize, Status>,
    ) -> Prayer {
        match self {
            Node::Parallel {
                children: _,
                needed_successed,
                failure_abort_limit,
            } => {
                let State::Parallel {
                    mut succeeded_children,
                    mut failed_children,
                    children_states_maybe,
                } = state
                else {
                    return Prayer::Status {
                        status: Status::Failure {
                            reason: format!("Excepted a Parallel state, got {state:?}"),
                        },
                    };
                };
                if children_states_maybe.is_some() {
                    return Prayer::Status {
                        status: Status::Failure {
                            reason: format!(
                                "Excepted a Parallel state, to not have any states for children"
                            ),
                        },
                    };
                }

                let children_states = childerns_returned_statuses
                    .into_iter()
                    .filter_map(|(idx, status)| match status {
                        Status::Success => {
                            succeeded_children.insert(idx);
                            None
                        }
                        Status::Failure { .. } => {
                            failed_children.insert(idx);
                            None
                        }
                        Status::Waiting { state } => Some((idx, Some(state))),
                    })
                    .collect();

                if &succeeded_children.len() >= needed_successed {
                    return Prayer::Status {
                        status: Status::Success,
                    };
                };
                if &failed_children.len() >= failure_abort_limit {
                    return Prayer::Status {
                        status: Status::Failure {
                            reason: format!("Too many child tasks failed"),
                        },
                    };
                };

                return Prayer::Status {
                    status: Status::Waiting {
                        state: State::Parallel {
                            succeeded_children,
                            failed_children,
                            children_states_maybe: Some(children_states),
                        },
                    },
                };
            }
            x @ Node::Selector { .. }
            | x @ Node::Sequence { .. }
            | x @ Node::Inverter { .. }
            | x @ Node::Combine { .. }
            | x @ Node::InventoryGE { .. } => {
                return Prayer::Status {
                    status: Status::Failure {
                        reason: format!("{} nodes are invalidfor multi_up_tick", x.name()),
                    },
                }
            }
        }
    }
}
