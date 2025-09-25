use qol::logy;

use crate::sandbox::ai::new_behavior_tree::{Node, Prayer, State, Status};

impl Node {
    pub fn up_tick(&self, state: State, childs_returned_status: Status) -> Prayer {
        match self {
            Node::Selector { children } => {
                let State::Selector { child_index, child_state_maybe, mut reason_for_failure } = state else {
                    return Prayer::Status { status: Status::Failure {reason: format!("Excepted a Selector state, got {state:?}")}}
                };
                if let Some(child_state) = child_state_maybe {
                    logy!("error", "Excepted a Selector state to not contain child_state, got {child_state:?}");
                };

                match childs_returned_status {
                    Status::Success => return Prayer::Status { status: Status::Success },
                    Status::Failure{ reason } => {
                        reason_for_failure.push_str("; ");
                        reason_for_failure.push_str(&reason);
                        let child_index = child_index + 1;
                        if child_index == children.len() {
                            return Prayer::Status { status: Status::Failure{reason: format!("All child tasks failed: {reason_for_failure}")} }
                        }
                        return Prayer::TickChild { 
                            child_index, 
                            my_state: State::Selector { child_index, child_state_maybe: None, reason_for_failure }, 
                            child_state_maybe: None }
                    },
                    Status::Waiting { state: child_state } => {
                        return Prayer::Status { 
                            status: Status::Waiting { 
                                state: State::Selector { 
                                    child_index, 
                                    child_state_maybe: Some(Box::new(child_state)),
                                    reason_for_failure,
                                }
                            }
                        }
                    },
                }
            },
            Node::Sequence { children } => {
                let State::Sequence { child_index, child_state_maybe } = state else {
                    return Prayer::Status { status: Status::Failure{reason: format!("Excepted a Sequence state, got {state:?}")} }
                };
                if let Some(child_state) = child_state_maybe {
                    logy!("error", "Excepted a Sequence state to not contain child_state, got {child_state:?}");
                };

                match childs_returned_status {
                    Status::Success => {
                        let child_index = child_index + 1;
                        if child_index == children.len() {
                            return Prayer::Status { status: Status::Success }
                        }
                        return Prayer::TickChild { 
                            child_index, 
                            my_state: State::Sequence { child_index, child_state_maybe: None }, 
                            child_state_maybe: None
                        }
                    },
                    Status::Failure{reason} => return Prayer::Status { status: Status::Failure{reason: format!("child task failed with: {reason}")} },
                    Status::Waiting { state: child_state } => {
                        return Prayer::Status { 
                            status: Status::Waiting { 
                                state: State::Sequence { 
                                    child_index, 
                                    child_state_maybe: Some(Box::new(child_state))
                                }
                            }
                        }
                    },
                }
            },
            Node::Inverter { child: _ } => {
                return Prayer::Status { status: match childs_returned_status {
                    Status::Success =>  Status::Failure{reason: "child succeeded".to_owned()},
                    Status::Failure{..} =>  Status::Success,
                    Status::Waiting { state: child_status } => {
                        Status::Waiting { state: State::Inverter { child_state_maybe: Some(Box::new(child_status)) } }
                    },
                }}
            },
            // Parallel should get the multi_up_tick()
            Node::Parallel{ .. }  => return Prayer::Status { status: Status::Failure{reason: format!("Parallel should only be up ticked with multi_up_tick()")} },
            x @ Node::Combine { .. } |
            x @ Node::InventoryGE { .. } => return Prayer::Status { status: Status::Failure{reason: format!("{} nodes doesn't have chrinren so should never be up ticked", x.name())} },
        }
    }
}