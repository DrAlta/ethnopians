use std::collections::HashMap;
use std::sync::Arc;

use ethnolib::ai::behavior_tree::*;
use ethnolib::ai::*;
use ethnolib::ai::test::Item;

type InpulseId = u8;
type EntityId = u32;
fn main() {
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
    let mut task_master = TaskMaster::<InpulseId, EntityId, Item>::new(
        blackboard,
        stack,
        0,
        None,
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

    let behavior_tree_tasks = HashMap::from([("test".to_owned(), tree)]);
    println!("{:#?}", task_master.tick(&behavior_tree_tasks, None));
}
