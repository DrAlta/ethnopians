use std::collections::{BTreeMap, BTreeSet};

use qol::logy;

use crate::sandbox::ai::{Blackboard, BlackboardKey, BlackboardValue};

mod prayer;
pub use prayer::Prayer;
mod status;
pub use status::Status;

mod state;
pub use state::State;

#[derive(Debug, Clone, PartialEq)]
pub enum Node {
    // theseare memory selector and memory sequence 
    Selector{children: Vec<Self>},
    Sequence{children: Vec<Self>},
    Parallel{children: Vec<Self>, needed_successed: usize, failure_abort_limit: usize},
    // #####
    // # Decorator
    // ###
    Inverter{child: Box<Self>},
    // #####
    // # Action
    // ###
    // takes two Blackboard keys that points to ItemClass
    Combine{key_to_direct_item_class: BlackboardKey, key_to_indirect_item_class: BlackboardKey},

    // #####
    // # Condition
    // ###
    // takes a Blackboard key that points to an ItemClass and u8 of the number to compare to
    InventoryGE{key_to_item_class: BlackboardKey, amount: i32},
}
impl Node {
    pub fn down_tick(
        &self, 
        state_maybe:Option<State>,
        blackboard: &mut Blackboard<BlackboardKey, BlackboardValue>,
    ) -> Prayer {
        match self {
            Node::Selector { children: _ } => {
                let (child_index, child_state_maybe) = if let Some(state) = state_maybe {
                    if let State::Selector { child_index, child_state_maybe } = state {
                        (
                            child_index, 
                            if let Some(child_state)= child_state_maybe {
                                Some(*child_state)
                            } else {
                                None
                            } )
                    } else {
                        logy!("error", "Excepted a Selector state, got {state:?}");
                        return Prayer::Status { status: Status::Failure }
                    }
                } else{
                    (0, None)
                };

                return Prayer::TickChild { 
                    child_index, 
                    my_state: State::Selector { child_index, child_state_maybe: None }, 
                    child_state_maybe
                }
            },
            Node::Sequence { children: _ } => {
                let (child_index, child_state_maybe) = if let Some(state) = state_maybe {
                    if let State::Selector { child_index, child_state_maybe } = state {
                        (
                            child_index, 
                            if let Some(child_state)= child_state_maybe {
                                Some(*child_state)
                            } else {
                                None
                            }
                        )
                    } else {
                        logy!("error", "Excepted a Sequence state, got {state:?}");
                        return Prayer::Status { status: Status::Failure }
                    }
                } else{
                    (0, None)
                };

                return Prayer::TickChild { 
                    child_index, 
                    my_state: State::Sequence { child_index, child_state_maybe: None }, 
                    child_state_maybe
                }
            },
            Node::Parallel { children, needed_successed, failure_abort_limit } => {
                if let Some(state) = &state_maybe {
                    if let State::Parallel { .. } = state {
                    } else {
                        logy!("error", "Excepted a Parallel state, got {state:?}");
                        return Prayer::Status { status: Status::Failure }
                    }
                }
                let (succeeded_children, failed_children, children_states_maybe) = if let Some(State::Parallel { succeeded_children, failed_children, children_states_maybe  }) = state_maybe {
                    (succeeded_children, failed_children, children_states_maybe)
                } else {
                    (BTreeSet::new(), BTreeSet::new(), None)
                };
                
                if &succeeded_children.len() >= needed_successed {
                    return Prayer::Status { status: Status::Success }
                };
                if &failed_children.len() >= failure_abort_limit {
                    return Prayer::Status { status: Status::Failure }
                };

                let children_states = if let Some(children_states) = children_states_maybe{
                    children_states
                 } else{ 
                    children.iter()
                        .enumerate()
                        .filter_map(
                            |(idx, _)|
                            {
                                if succeeded_children.contains(&idx) || failed_children.contains(&idx) {
                                    Some((idx, None))
                                } else {
                                    None
                                }
                            }
                        ).collect()
                    };
                return Prayer::TickChildren { children_states }
            },
            Node::Inverter { child: _ } => {
                // Inverter: invert child's terminal results, propagate child ticking/waiting and preserve child state
                if let Some(state) = &state_maybe {
                    logy!("error", "Excepted an Inverter state, got {state:?}");
                    return Prayer::Status{status: Status::Failure}
                };

                let child_state_maybe = if let Some(State::Inverter { child_state_maybe }) = state_maybe {
                    if let Some(x) =child_state_maybe {
                        Some(*x)
                    } else {
                        None
                    }
                }else {
                    return Prayer::Status { status: Status::Failure }
                };
                
                return Prayer::TickChild { child_index: 0, my_state: State::Inverter { child_state_maybe: None }, child_state_maybe }
            },
            Node::Combine { key_to_direct_item_class, key_to_indirect_item_class } => {
                let Some(BlackboardValue::String(direct_item_class_string)) = blackboard.get(key_to_direct_item_class) else {
                    return Prayer::Status { status: Status::Failure } //Err(format!("{key} not found in blackboard"));
                };

                let direct_item_class_str: &str = &direct_item_class_string;
                let Ok(direct_item_class) = direct_item_class_str.try_into() else {
                    return Prayer::Status { status: Status::Failure } //Err(format!("{item_class_string} is not a valid item class"));
                };


                let Some(BlackboardValue::String(indirect_item_class_string)) = blackboard.get(key_to_indirect_item_class) else {
                    return Prayer::Status { status: Status::Failure } //Err(format!("{key} not found in blackboard"));
                };

                let indirect_item_class_str: &str = &indirect_item_class_string;
                let Ok(indirect_item_class) = indirect_item_class_str.try_into() else {
                    return Prayer::Status { status: Status::Failure } //Err(format!("{item_class_string} is not a valid item class"));
                };

                return Prayer::Combine { direct_item_class, indirect_item_class }
            },
            Node::InventoryGE { key_to_item_class, amount } => {
                let Some(&BlackboardValue::EntityId(agent)) = blackboard.get("self") else {
                    return Prayer::Status { status: Status::Failure } // Err(format!("self not found in blackboard"));
                };
                let Some(BlackboardValue::String(item_class_string)) = blackboard.get(key_to_item_class) else {
                    return Prayer::Status { status: Status::Failure } //Err(format!("{key} not found in blackboard"));
                };

                let item_class_str: &str = &item_class_string;
                let Ok(item_class) = item_class_str.try_into() else {
                    return Prayer::Status { status: Status::Failure } //Err(format!("{item_class_string} is not a valid item class"));
                };

                return Prayer::GetIsInventoryGE {
                    agent,
                    item_class,
                    amount: *amount,
                }
            },
        }
    }
    pub fn up_tick(&self, state: State, childs_returned_status: Status) -> Prayer {
        match self {
            Node::Selector { children } => {
                let State::Selector { child_index, child_state_maybe } = state else {
                    logy!("error", "Excepted a Selector state, got {state:?}");
                    return Prayer::Status { status: Status::Failure }
                };
                if let Some(child_state) = child_state_maybe {
                    logy!("error", "Excepted a Selector state to not contain child_state, got {child_state:?}");
                };

                match childs_returned_status {
                    Status::Success => return Prayer::Status { status: Status::Success },
                    Status::Failure => {
                        let child_index = child_index + 1;
                        if child_index == children.len() {
                            return Prayer::Status { status: Status::Failure }
                        }
                        return Prayer::TickChild { 
                            child_index, 
                            my_state: State::Selector { child_index, child_state_maybe: None }, 
                            child_state_maybe: None }
                    },
                    Status::Waiting { state: child_state } => {
                        return Prayer::Status { 
                            status: Status::Waiting { 
                                state: State::Selector { 
                                    child_index, 
                                    child_state_maybe: Some(Box::new(child_state))
                                }
                            }
                        }
                    },
                }
            },
            Node::Sequence { children } => {
                let State::Sequence { child_index, child_state_maybe } = state else {
                    logy!("error", "Excepted a Sequence state, got {state:?}");
                    return Prayer::Status { status: Status::Failure }
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
                            my_state: State::Selector { child_index, child_state_maybe: None }, 
                            child_state_maybe: None
                        }
                    },
                    Status::Failure => return Prayer::Status { status: Status::Failure },
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
                    Status::Success =>  Status::Failure,
                    Status::Failure =>  Status::Success,
                    Status::Waiting { state: child_status } => {
                        Status::Waiting { state: State::Inverter { child_state_maybe: Some(Box::new(child_status)) } }
                    },
                }}
            },
            // Parallel should get the multi_up_tick()
            Node::Parallel{ .. } |
            Node::Combine { .. } |
            Node::InventoryGE { .. } => return Prayer::Status { status: Status::Failure },
        }
    }
    pub fn multi_up_tick(&self, state: State, childerns_returned_statuses: BTreeMap<usize, Status>) -> Prayer {
        match self {
            Node::Parallel { children: _, needed_successed, failure_abort_limit } => {
                let State::Parallel { mut succeeded_children, mut failed_children, children_states_maybe } = state else {
                    logy!("error", "Excepted a Parallel state, got {state:?}");
                    return Prayer::Status { status: Status::Failure }
                };
                if children_states_maybe.is_some() {
                    logy!("error", "Excepted a Parallel state, to not have any states for children");
                    return Prayer::Status { status: Status::Failure }
                }

                let children_states = childerns_returned_statuses.into_iter().filter_map(
                    |(idx, status)|
                    {
                        match status {
                            Status::Success => {
                                succeeded_children.insert(idx);
                                None
                            },
                            Status::Failure => {
                                failed_children.insert(idx);
                                None
                            },
                            Status::Waiting { state } => Some((idx, Some(state))),
                        }
                    }
                ).collect();
                
                if &succeeded_children.len() >= needed_successed {
                    return Prayer::Status { status: Status::Success }
                };
                if &failed_children.len() >= failure_abort_limit {
                    return Prayer::Status { status: Status::Failure }
                };

                return Prayer::Status { status: Status::Waiting { state: State::Parallel { 
                    succeeded_children, 
                    failed_children,
                    children_states_maybe: Some(children_states),
                 } } }

            },
            Node::Selector { .. } |
            Node::Sequence { .. } |
            Node::Inverter { .. } |
            Node::Combine { .. } |
            Node::InventoryGE { .. } => return Prayer::Status { status: Status::Failure },
        }
    }

}