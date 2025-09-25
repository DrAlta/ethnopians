use std::sync::Arc;

use qol::assert_specimen;

use crate::sandbox::ai::{new_behavior_tree::*, Blackboard, BlackboardValue, Variable};

#[test]
fn test(){
    let mut blackboard = Blackboard::new();
    blackboard.insert(
        "A".to_owned(), 
        Variable::Chit(BlackboardValue::String(Arc::new("Veggie".to_owned()))),
    );
    blackboard.insert(
        "B".to_owned(), 
        Variable::Chit(BlackboardValue::String(Arc::new("Axe".to_owned()))),
    );
    let tree= Node::Sequence { children: vec![
        Node::Combine { key_to_direct_item_class: "A".to_owned(), key_to_indirect_item_class: "B".to_owned() },
        Node::InventoryGE { key_to_item_class: "B".to_owned(), amount: 1 }
    ] };

    let x1 = tree.down_tick(None, &mut blackboard);
    assert_specimen!(
        &x1,
        &Prayer::TickChild {
            child_index: 0,
            my_state: State::Sequence {
                child_index: 0,
                child_state_maybe: None
            },
            child_state_maybe: None
        }
    );
    let Prayer::TickChild { child_index, my_state, child_state_maybe } = x1 else {panic!()};
    let x2;
    {
    let Node::Sequence { children } = &tree else {panic!()};
    x2 = children[child_index].down_tick(child_state_maybe, &mut blackboard)
    }
    panic!("{x2:?}")
}