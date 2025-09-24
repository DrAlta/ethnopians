use std::collections::HashMap;

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
    Selector{child_index: usize, child_state: Box<Self>},
    Sequence{child_index: usize, child_state: Box<Self>},
    Parallel{child_states: HashMap<usize, Self>},

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
    Parallel{children: Vec<Self>},
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
                    if let State::Selector { child_index, child_state } = state {
                    } else {
                        logy!("error", "Excepted a Selector state, got {state:?}");
                    }
                }
                let (skip, mut child_state_maybe) = if let Some(State::Selector { child_index, child_state }) = state_maybe {
                    (child_index -1, Some(*child_state))
                } else {
                    (0, None)
                };
                for (index, child) in children.iter().enumerate().skip(skip) {
                    match child.tick(child_state_maybe) {
                        Status::Success => return Status::Success,
                        Status::Failure => {
                            child_state_maybe = None; 
                            continue
                        },
                        Status::Running { state } => {
                            return Status::Running { state: State::Selector { child_index: index, child_state: Box::new(state) } }
                        },
                    }
                }
                return Status::Failure
            },
            Node::Sequence { children } => {
                if let Some(state) = &state_maybe {
                    if let State::Sequence { child_index, child_state } = state {
                    } else {
                        logy!("error", "Excepted a Sequence state, got {state:?}");
                    }
                }
                let (skip, mut child_state_maybe) = if let Some(State::Sequence { child_index, child_state }) = state_maybe {
                    (child_index -1, Some(*child_state))
                } else {
                    (0, None)
                };
                for (index, child) in children.iter().enumerate().skip(skip) {
                    match child.tick(child_state_maybe) {
                        Status::Success => {
                            child_state_maybe = None; 
                            continue
                        },
                        Status::Failure => return Status::Failure,
                        Status::Running { state } => {
                            return Status::Running { state: State::Sequence { child_index: index, child_state: Box::new(state) } }
                        },
                    }
                }
                return Status::Success
            },
            Node::Parallel { children } => todo!(),
            Node::Inverter { child } => todo!(),
            Node::Combine { key_to_direct_item_class, key_to_indirect_item_class } => todo!(),
            Node::InventoryGE { key_to_item_class, amount } => todo!(),
        }
    }
}