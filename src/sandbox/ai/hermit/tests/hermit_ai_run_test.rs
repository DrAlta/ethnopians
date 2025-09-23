use std::{collections::BTreeMap, sync::Arc};

use crate::sandbox::{
    ai::{
        get_hermit_behavior_task, task_testing_harness, Blackboard, BlackboardValue, StackItem,
        Variable,
    },
    EntityId, Item,
};

#[test]
fn hermit_test() {
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
        "food".to_owned(),
        Variable::Chit(BlackboardValue::String(Arc::new("Veggie".to_owned()))),
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
    let find_in_inventory = vec![
        EntityId::from(70_u64),
        EntityId::from(71_u64),
        EntityId::from(72_u64),
        EntityId::from(73_u64),
        EntityId::from(74_u64),
        EntityId::from(75_u64),
        EntityId::from(76_u64),
        EntityId::from(77_u64),
        EntityId::from(78_u64),
        EntityId::from(79_u64),
        EntityId::from(80_u64),
        EntityId::from(81_u64),
        EntityId::from(82_u64),
        EntityId::from(83_u64),
        EntityId::from(84_u64),
        EntityId::from(85_u64),
        EntityId::from(86_u64),
        EntityId::from(87_u64),
        EntityId::from(88_u64),
        EntityId::from(89_u64),
    ];
    let find_nearest = vec![house, house, house];
    let get_entities = vec![
        [
            (StackItem::Int(0), StackItem::EntityId(garden_1)),
            (StackItem::Int(1), StackItem::EntityId(garden_2)),
            (StackItem::Int(2), StackItem::EntityId(garden_3)),
            (StackItem::Int(3), StackItem::EntityId(garden_4)),
        ]
        .try_into()
        .unwrap(),
        [
            (StackItem::Int(0), StackItem::EntityId(garden_1)),
            (StackItem::Int(1), StackItem::EntityId(garden_2)),
            (StackItem::Int(2), StackItem::EntityId(garden_3)),
            (StackItem::Int(3), StackItem::EntityId(garden_4)),
        ]
        .try_into()
        .unwrap(),
        [
            (StackItem::Int(0), StackItem::EntityId(garden_1)),
            (StackItem::Int(1), StackItem::EntityId(garden_2)),
            (StackItem::Int(2), StackItem::EntityId(garden_3)),
            (StackItem::Int(3), StackItem::EntityId(garden_4)),
        ]
        .try_into()
        .unwrap(),
        [
            (StackItem::Int(0), StackItem::EntityId(garden_1)),
            (StackItem::Int(1), StackItem::EntityId(garden_2)),
            (StackItem::Int(2), StackItem::EntityId(garden_3)),
            (StackItem::Int(3), StackItem::EntityId(garden_4)),
        ]
        .try_into()
        .unwrap(),
        [
            (StackItem::Int(0), StackItem::EntityId(garden_1)),
            (StackItem::Int(1), StackItem::EntityId(garden_2)),
            (StackItem::Int(2), StackItem::EntityId(garden_3)),
            (StackItem::Int(3), StackItem::EntityId(garden_4)),
        ]
        .try_into()
        .unwrap(),
        [
            (StackItem::Int(0), StackItem::EntityId(garden_1)),
            (StackItem::Int(1), StackItem::EntityId(garden_2)),
            (StackItem::Int(2), StackItem::EntityId(garden_3)),
            (StackItem::Int(3), StackItem::EntityId(garden_4)),
        ]
        .try_into()
        .unwrap(),
    ];
    let get_energy = vec![4];

    let get_location = vec![
        (6, 1),
        (6, 2),
        (6, 3),
        (6, 4),
        (6, 5),
        (6, 6),
        (6, 7),
        (6, 8),
        (6, 9),
        (6, 10),
    ];
    let get_hp = vec![4];
    let get_is_inventory_ge = vec![false, true, true]; // true];
    let running = vec![
        true, true, true, true, true, true, true, true, true, true, true, true, true, true, true,
        true, true, true, true, true, true, true, true, true, true, true, true, true, true, true,
        true, true, true, true, true, true, true, true, true, true, true, true, true, true, true,
        true, true, true, true, true, true,
    ];
    // done setting up dummy values

    let task_db = get_hermit_behavior_task();

    task_testing_harness(
        "hermit",
        task_db,
        vec![StackItem::success()],
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
