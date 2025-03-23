use std::collections::HashMap;

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
    let my_self = EntityId::from_raw(0);
    let house = EntityId::from_raw(5);
    let garden_1 = EntityId::from_raw(51);
    let garden_2 = EntityId::from_raw(52);
    let garden_3 = EntityId::from_raw(52);
    let garden_4 = EntityId::from_raw(54);
    let item_types: HashMap<bevy::ecs::entity::Entity, Item> = HashMap::from([
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
        Variable::Chit(BlackboardValue::String("Veggie".to_owned())),
    );
    blackboard.insert(
        // I should change the GetIsInventoryGE to hold a BlackBoardValue instadt of a key
        "knife".to_owned(),
        Variable::Chit(BlackboardValue::String("Knife".to_owned())),
    );
    blackboard.insert(
        // I should change the GetIsInventoryGE to hold a BlackBoardValue instadt of a key
        "stone".to_owned(),
        Variable::Chit(BlackboardValue::String("Stone".to_owned())),
    );

    // set up  the dummy values
    let find_in_inventory = vec![
        EntityId::from_raw(70),
        EntityId::from_raw(71),
        EntityId::from_raw(72),
        EntityId::from_raw(73),
        EntityId::from_raw(74),
        EntityId::from_raw(75),
        EntityId::from_raw(76),
        EntityId::from_raw(77),
        EntityId::from_raw(78),
        EntityId::from_raw(79),
        EntityId::from_raw(80),
        EntityId::from_raw(81),
        EntityId::from_raw(82),
        EntityId::from_raw(83),
        EntityId::from_raw(84),
        EntityId::from_raw(85),
        EntityId::from_raw(86),
        EntityId::from_raw(87),
        EntityId::from_raw(88),
        EntityId::from_raw(89),
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
