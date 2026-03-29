use std::collections::HashMap;
use std::sync::Arc;

use qol::assert_specimen;

use crate::ai::behavior_tree::*;
use crate::ai::task_master::sub_system_state::Branch;
use crate::ai::task_master::SubSystemState;
use crate::ai::test::Item;
use crate::ai::{Blackboard, BlackboardValue, Prayer, Status, TaskMaster, Variable};

type InpulseId = u8;
type EntityId = u32;

#[test]
fn behavior_treee_test_of_task_master() {
    let mut blackboard = Blackboard::new();
    blackboard.insert(
        "self".to_owned(),
        Variable::Chit(BlackboardValue::<EntityId>::EntityId(42_u32)),
    );
    blackboard.insert(
        "A".to_owned(),
        Variable::Chit(BlackboardValue::String(Arc::new("Veggie".to_owned()))),
    );
    blackboard.insert(
        "B".to_owned(),
        Variable::Chit(BlackboardValue::String(Arc::new("Axe".to_owned()))),
    );

    let stack = Vec::from([SubSystemState::BehaviorTree {
        tree_id: "test".to_owned(),
        execution_limb: Vec::new(),
        state_tick_next_maybe: None,
    }]);
    let mut task_master = TaskMaster::<InpulseId, EntityId, Item> {
        tick_counter: 0,
        blackboard,
        stack,
        prayer_being_waited_on_maybe: None,
    };

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

    let behavior_tree_tasks = HashMap::from([("test".to_owned(), tree)]);
    let x1 = task_master.tick(&behavior_tree_tasks, None);
    assert_specimen!(x1, crate::ai::TastMasterReport::Ok);
    let SubSystemState::BehaviorTree {
        tree_id: _,
        execution_limb,
        state_tick_next_maybe: _,
    } = task_master.stack.last().unwrap()
    else {
        panic!()
    };
    let b1 = execution_limb.last().unwrap();
    assert_specimen!(
        b1,
        &Branch {
            child_index: 0,
            parent_up_tick_state: State::Sequence {
                child_index: 0,
                child_state_maybe: None
            },
        }
    );

    let x2 = task_master.tick(&behavior_tree_tasks, None);
    assert_specimen!(
        x2,
        crate::ai::TastMasterReport::Prayer(Prayer::Combine {
            direct_item_class: Item::Veggie,
            indirect_item_class: Item::Axe
        })
    );
    let SubSystemState::BehaviorTree {
        tree_id: _,
        execution_limb,
        state_tick_next_maybe: _,
    } = task_master.stack.last().unwrap()
    else {
        panic!()
    };
    let b2 = execution_limb.last().unwrap();
    assert_specimen!(
        b2,
        &Branch {
            child_index: 0,
            parent_up_tick_state: State::Sequence {
                child_index: 0,
                child_state_maybe: None
            },
        }
    );

    let x3 = task_master.tick(
        &behavior_tree_tasks,
        Some(ExecReport::Status {
            status: Status::Success,
        }),
    );
    assert_specimen!(x3, crate::ai::TastMasterReport::Ok);

    let SubSystemState::BehaviorTree {
        tree_id: _,
        execution_limb,
        state_tick_next_maybe: _,
    } = task_master.stack.last().unwrap()
    else {
        panic!()
    };
    let b3 = execution_limb.last().unwrap();
    assert_specimen!(
        b3,
        &Branch {
            child_index: 1,
            parent_up_tick_state: State::Sequence {
                child_index: 1,
                child_state_maybe: None
            },
        }
    );

    let x4 = task_master.tick(&behavior_tree_tasks, None);
    assert_specimen!(
        x4,
        crate::ai::TastMasterReport::Prayer(Prayer::GetIsInventoryGE {
            agent: 42_u32.into(),
            item_class: Item::Axe,
            amount: 1
        })
    );

    let x5 = task_master.tick(
        &behavior_tree_tasks,
        Some(ExecReport::Status {
            status: Status::Success,
        }),
    );
    assert_specimen!(x5, crate::ai::TastMasterReport::Success);
}
