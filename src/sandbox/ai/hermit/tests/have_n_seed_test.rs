use std::{collections::BTreeMap, sync::Arc};

use crate::sandbox::{
    ai::{
        get_hermit_behavior_task, task_testing_harness, Blackboard, BlackboardValue, Instruction,
        StackItem, Variable,
    },
    EntityId, Item,
};

#[test]
fn have_n_seed_test() {
    // Set up the world
    let my_self = EntityId::from(0_u64);
    let house = EntityId::from(5_u64);
    let garden_1 = EntityId::from(51_u64);
    let garden_2 = EntityId::from(52_u64);
    let garden_3 = EntityId::from(52_u64);
    let garden_4 = EntityId::from(54_u64);
    let item_types: BTreeMap<EntityId, Item> = BTreeMap::from([
        (my_self, Item::Agent),
        (house, Item::House),
        (garden_1, Item::Stone),
        (garden_2, Item::Stone),
        (garden_3, Item::Stone),
        (garden_4, Item::Stone),
    ]);

    let mut blackboard: Blackboard<String, BlackboardValue> = Blackboard::new();
    blackboard.insert(
        "self".to_owned(),
        Variable::Chit(BlackboardValue::EntityId(my_self)),
    );
    blackboard.insert(
        // I should change the GetIsInventoryGE to hold a BlackBoardValue instadt of a key
        "knife".to_owned(),
        Variable::Chit(BlackboardValue::String(Arc::new("Knife".to_owned()))),
    );
    blackboard.insert(
        // I should change the GetIsInventoryGE to hold a BlackBoardValue instadt of a key
        "stone".to_owned(),
        Variable::Chit(BlackboardValue::String(Arc::new("Stone".to_owned()))),
    );

    // set up  the dummy values
    let find_in_inventory = vec![EntityId::from(70_u64)];
    let find_nearest = vec![];
    let get_entities = vec![];
    let get_energy = vec![];

    let get_location = vec![];
    let get_hp = vec![];
    let get_is_inventory_ge = vec![false, true, true];
    let running = vec![true];

    let mut task_db = get_hermit_behavior_task();
    task_db.insert(
        "have_n_seed_test".to_owned(),
        vec![
            Instruction::ForthDrop,
            Instruction::ForthLit(StackItem::Int(1)),
            Instruction::ForthCall("have_n_seed".to_owned(), 0),
            Instruction::ForthReturn,
        ],
    );

    task_testing_harness(
        "have_n_seed_test",
        task_db,
        vec![StackItem::True],
        find_in_inventory,
        find_nearest,
        get_entities,
        get_energy,
        get_location,
        get_hp,
        get_is_inventory_ge,
        running,
        blackboard,
        item_types,
    )
}
