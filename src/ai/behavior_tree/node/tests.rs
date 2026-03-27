use std::sync::Arc;

use qol::assert_specimen;

use crate::sandbox::{
    Item, new_ai::{Blackboard, BlackboardValue, Prayer, Status, Variable, behavior_tree::*}
};

#[test]
fn test() {
    let mut blackboard = Blackboard::new();
    blackboard.insert(
        "self".to_owned(),
        Variable::Chit(BlackboardValue::EntityId(42_u64.into())),
    );
    blackboard.insert(
        "A".to_owned(),
        Variable::Chit(BlackboardValue::String(Arc::new("Veggie".to_owned()))),
    );
    blackboard.insert(
        "B".to_owned(),
        Variable::Chit(BlackboardValue::String(Arc::new("Axe".to_owned()))),
    );
    let tree = Node::Sequence {
        children: vec![
            Node::Combine {
                key_to_direct_item_class: "A".to_owned(),
                key_to_indirect_item_class: "B".to_owned(),
            },
            Node::InventoryGE {
                key_to_item_class: "B".to_owned(),
                amount: 1,
            },
        ],
    };

    let x1 = tree.down_tick(None, &mut blackboard);
    assert_specimen!(
        &x1,
        &ExecReport::TickChild {
            child_index: 0,
            my_state: State::Sequence {
                child_index: 0,
                child_state_maybe: None
            },
            child_state_maybe: None
        }
    );
    let Node::Sequence { children } = &tree else {
        panic!()
    };

    let ExecReport::TickChild {
        child_index,
        my_state,
        child_state_maybe,
    } = x1
    else {
        panic!()
    };
    let x2;
    x2 = children[child_index].down_tick(child_state_maybe, &mut blackboard);
    assert_specimen!(
        x2,
        ExecReport::Prayer(Prayer::Combine {
            direct_item_class: Item::Veggie,
            indirect_item_class: Item::Axe
        })
    );
    // I'm leaning for when a condition prayer is made the sky daddy up ticks the parent with the answer
    let x3 = tree.up_tick(my_state, Status::Success);
    let ExecReport::TickChild {
        child_index,
        my_state,
        child_state_maybe,
    } = x3
    else {
        panic!()
    };
    let x4 = children[child_index].down_tick(child_state_maybe, &mut blackboard);
    assert_specimen!(
        x4,
        ExecReport::Prayer(Prayer::GetIsInventoryGE {
            agent: 42_u64.into(),
            item_class: Item::Axe,
            amount: 1
        })
    );
    let x5 = tree.up_tick(my_state, Status::Success);
    assert_specimen!(
        x5,
        ExecReport::Status {
            status: Status::Success
        }
    );
}
