use std::{collections::BTreeMap, sync::Arc};

use crate::sandbox::{
    ai::{
        get_hermit_behavior_task, task_testing_harness, Blackboard, BlackboardValue, Instruction,
        StackItem, Variable,
    },
    EntityId, Item,
};

#[test]
fn check_if_clear_for_garden_false_test() {
    // Set up the world
    let my_self = EntityId::from_raw(0);
    let house = EntityId::from_raw(5);
    let garden_1 = EntityId::from_raw(51);
    let garden_2 = EntityId::from_raw(52);
    let garden_3 = EntityId::from_raw(52);
    let garden_4 = EntityId::from_raw(54);
    let item_types: BTreeMap<bevy::ecs::entity::Entity, Item> = BTreeMap::from([
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
        "food".to_owned(),
        Variable::Chit(BlackboardValue::String(Arc::new("Veggie".to_owned()))),
    );

    // set up  the dummy values
    let find_in_inventory = vec![];
    let find_nearest = vec![];
    let get_entities = vec![[
        (StackItem::Int(0), StackItem::EntityId(garden_1)),
        (StackItem::Int(1), StackItem::EntityId(garden_2)),
        (StackItem::Int(2), StackItem::EntityId(garden_3)),
        (StackItem::Int(3), StackItem::EntityId(garden_4)),
    ]
    .try_into()
    .unwrap()];
    let get_energy = vec![];

    let get_location = vec![];
    let get_hp = vec![];
    let get_is_inventory_ge = vec![];
    let running = vec![];

    let mut task_db = get_hermit_behavior_task();

    task_db.insert(
        "check_if_clear_for_garden_test".to_owned(),
        vec![
            Instruction::ForthDrop,
            Instruction::ForthLit(StackItem::Coord { x: 4, y: 2 }),
            Instruction::ForthJump("check_if_clear_for_garden".to_owned(), 0),
        ],
    );

    task_testing_harness(
        "check_if_clear_for_garden_test",
        task_db,
        vec![StackItem::False],
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
#[test]
fn check_if_clear_for_garden_true_test() {
    // Set up the world
    let my_self = EntityId::from_raw(0);
    let house = EntityId::from_raw(5);
    let garden_1 = EntityId::from_raw(51);
    let garden_2 = EntityId::from_raw(52);
    let garden_3 = EntityId::from_raw(52);
    let garden_4 = EntityId::from_raw(54);
    let item_types: BTreeMap<bevy::ecs::entity::Entity, Item> = BTreeMap::from([
        (my_self, Item::Agent),
        (house, Item::House),
        (garden_1, Item::Veggie),
        (garden_2, Item::Agent),
        (garden_3, Item::Veggie),
        (garden_4, Item::Agent),
    ]);

    let mut blackboard: Blackboard<String, BlackboardValue> = Blackboard::new();
    blackboard.insert(
        "self".to_owned(),
        Variable::Chit(BlackboardValue::EntityId(my_self)),
    );
    blackboard.insert(
        "food".to_owned(),
        Variable::Chit(BlackboardValue::String(Arc::new("Veggie".to_owned()))),
    );

    // set up  the dummy values
    let find_in_inventory = vec![];
    let find_nearest = vec![];
    let get_entities = vec![[
        (StackItem::Int(0), StackItem::EntityId(garden_1)),
        (StackItem::Int(1), StackItem::EntityId(garden_2)),
        (StackItem::Int(2), StackItem::EntityId(garden_3)),
        (StackItem::Int(3), StackItem::EntityId(garden_4)),
    ]
    .try_into()
    .unwrap()];
    let get_energy = vec![];

    let get_location = vec![];
    let get_hp = vec![];
    let get_is_inventory_ge = vec![];
    let running = vec![];

    let mut task_db = get_hermit_behavior_task();

    task_db.insert(
        "check_if_clear_for_garden_test".to_owned(),
        vec![
            Instruction::ForthDrop,
            Instruction::ForthLit(StackItem::Coord { x: 4, y: 2 }),
            Instruction::ForthJump("check_if_clear_for_garden".to_owned(), 0),
        ],
    );

    task_testing_harness(
        "check_if_clear_for_garden_test",
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
