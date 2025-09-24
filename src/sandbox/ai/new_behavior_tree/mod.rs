use qol::logy;

use crate::sandbox::ai::BlackboardKey;
#[derive(Debug, Clone, PartialEq)]
pub enum Status {
    Success,
    Failure,
    Running{state: State}
}
#[derive(Debug, Clone, PartialEq)]
pub enum State {
    Selector{child_index: usize, child_state_maybe: Option<Box<Self>>},
    Sequence{child_index: usize, child_state_maybe: Option<Box<Self>>},
    Parallel{child_statuses: Vec<Status>},

    // #####
    // # Decorator
    // ###
    Inverter{child_state: Box<Self>},

    // #####
    // # Action
    // ###
    Combine,

    // #####
    // # Condition
    // ###
    InventoryGE,

}

#[derive(Debug, Clone, PartialEq)]
pub enum Node {
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
    pub fn tick(&self, state_maybe:Option<State>) -> Status {
        match self {
            Node::Selector { children } => {
                if let Some(state) = &state_maybe {
                    if let State::Selector { child_index: _, child_state_maybe: _ } = state {
                    } else {
                        logy!("error", "Excepted a Selector state, got {state:?}");
                    }
                }
                let (child_index, child_state_maybe) = if let Some(State::Selector { child_index, child_state_maybe }) = state_maybe {
                    let child_state_maybe = if let Some(child_state) = child_state_maybe {
                        Some(*child_state)
                    } else {
                        None
                    };

                    (child_index , child_state_maybe)
                } else {
                    (0, None)
                };
                let Some(child) = children.get(child_index) else {
                    logy!("error", "failed to get child{child_index}");
                    return Status::Failure
                };
                match child.tick(child_state_maybe) {
                    Status::Success => return Status::Success,
                    Status::Failure => {
                        let child_index = child_index + 1;
                        if child_index == children.len() {
                            return Status::Success
                        }
                        let child_state_maybe= None;
                        return Status::Running { state: State::Selector { child_index, child_state_maybe } }
                    },
                    Status::Running { state } => {
                        return Status::Running { state: State::Selector { child_index, child_state_maybe: Some(Box::new(state)) } }
                    },
                }
            },
            Node::Sequence { children } => {
                if let Some(state) = &state_maybe {
                    if let State::Sequence { child_index: _ , child_state_maybe: _ } = state {
                    } else {
                        logy!("error", "Excepted a Sequence state, got {state:?}");
                    }
                }
                let (child_index, child_state_maybe) = if let Some(State::Sequence { child_index, child_state_maybe }) = state_maybe {
                    let child_state_maybe = if let Some(child_state) = child_state_maybe {
                        Some(*child_state)
                    } else {
                        None
                    };

                    (child_index , child_state_maybe)
                } else {
                    (0, None)
                };
                let Some(child) = children.get(child_index) else {
                    logy!("error", "failed to get child{child_index}");
                    return Status::Failure
                };
                match child.tick(child_state_maybe) {
                    Status::Success => {
                        let child_index = child_index + 1;
                        if child_index == children.len() {
                            return Status::Success
                        }
                        let child_state_maybe= None;
                        return Status::Running { state: State::Sequence { child_index, child_state_maybe } }
                    },
                    Status::Failure => return Status::Failure,
                    Status::Running { state } => {
                        return Status::Running { state: State::Sequence { child_index, child_state_maybe: Some(Box::new(state)) } }
                    },
                }
            },
            Node::Parallel { children, needed_successed, failure_abort_limit } => {
                if let Some(state) = &state_maybe {
                    if let State::Parallel { .. } = state {
                    } else {
                        logy!("error", "Excepted a Parallel state, got {state:?}");
                    }
                }
                let child_statuses = if let Some(State::Parallel { child_statuses  }) = state_maybe {
                    child_statuses
                } else {
                    return parallel(
                        children.iter().map(|child| child.tick(None)).collect(),
                        *needed_successed, *failure_abort_limit
                    )
                };
                
                if children.len() != child_statuses.len() {
                    logy!("error", "children{} not eqaul child_states{}",children.len(), child_statuses.len());
                    return Status::Failure;
                }

                let child_statuses: Vec<Status> = children.iter()
                    .zip(child_statuses)
                    .map(
                        |(child, status)| 
                        {
                            match status {
                                Status::Success => Status::Success,
                                Status::Failure => Status::Failure,
                                Status::Running { state } => child.tick(Some(state)),
                            }
                        }
                    ).collect();
        
                return parallel(child_statuses, *needed_successed, *failure_abort_limit)

            },
            Node::Inverter { child } => todo!(),
            Node::Combine { key_to_direct_item_class, key_to_indirect_item_class } => todo!(),
            Node::InventoryGE { key_to_item_class, amount } => todo!(),
        }
    }
}

fn parallel(child_statuses:Vec<Status>, needed_successed: usize, failure_abort_limit: usize ) -> Status{
    let mut successes = 0;
    let mut failures = 0;
    let mut cont= false;
    child_statuses.iter().for_each(|x|{
        match x {
            Status::Success => {successes +=1;},
            Status::Failure => {failures +=1;},
            Status::Running { state: _ } => {
                cont = true
            },
        };
    });
    if successes >= needed_successed {
        return Status::Success
    };
    if failures >= failure_abort_limit {
        return Status::Failure
    };
    if cont {
        return Status::Running { state: State::Parallel { child_statuses } }
    } else {
        return Status::Failure
    }

}