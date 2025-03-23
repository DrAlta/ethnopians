use std::collections::HashMap;

use crate::sandbox::{
    ai::{
        get_hermit_behavior_task, task_testing_harness, Blackboard, BlackboardValue, StackItem,
        Variable,
    },
    EntityId, Item,
};

#[test]
fn harvest_veg_test() {
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
        "garden_location".to_owned(),
        Variable::Chit(BlackboardValue::Coord { x: 4, y: 2 }),
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

    let task_db = get_hermit_behavior_task();

    task_testing_harness(
        "harvest_veg",
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
