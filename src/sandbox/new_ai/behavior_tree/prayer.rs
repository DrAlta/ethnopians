use std::collections::BTreeMap;

use crate::sandbox::{EntityId, Item};

use super::{State, Status};

#[derive(Debug, Clone, PartialEq)]

pub enum Prayer {
    TickChild{child_index: usize, my_state: State, child_state_maybe: Option<State>},
    TickChildren{children_states: BTreeMap<usize, Option<State>>},
    Status{status: Status},
    // #####
    // # Action
    // ###
    Combine{direct_item_class:Item, indirect_item_class: Item},

    // #####
    // # Condition
    // ###
    // takes a Blackboard key that points to an ItemClass and u8 of the number to compare to
    GetIsInventoryGE {
        agent: EntityId,
        item_class: Item,
        amount: i32,
    },
}
