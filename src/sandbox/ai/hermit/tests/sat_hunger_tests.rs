use std::collections::BTreeMap;

use crate::sandbox::{
    ai::{
        get_hermit_behavior_task, task_testing_harness, Blackboard, BlackboardValue, StackItem,
        Variable,
    },
    EntityId, Item,
};

// "energy is grather that 5"
#[test]
fn sat_hunger_test_t() {
    // set up the world
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
    blackboard.insert("food".to_owned(), Variable::Chit("Veggie".into()));

    // set up  the dummy values
    let find_in_inventory = vec![];
    let find_nearest = vec![];
    let get_entities = vec![];
    let get_energy = vec![7];

    let get_location = vec![];
    let get_hp = vec![];
    let get_is_inventory_ge = vec![];
    let running = vec![];

    let task_db = get_hermit_behavior_task();

    task_testing_harness(
        "sat_hunger",
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
// 'got 4 for energy', "energy not greater than 5", 'greater than or equal than 1 food in invitory', 'eat successful'
#[test]
fn sat_hunger_test_ft() {
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
    blackboard.insert("food".to_owned(), Variable::Chit("Veggie".into()));

    // set up  the dummy values
    let find_in_inventory = vec![];
    let find_nearest = vec![];
    let get_entities = vec![];
    let get_energy = vec![4];

    let get_location = vec![];
    let get_hp = vec![];
    let get_is_inventory_ge = vec![true];
    let running = vec![true];

    let task_db = get_hermit_behavior_task();

    task_testing_harness(
        "sat_hunger",
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
// 'got 4 for energy', "energy not greater than 5", 'not greater than or equal than 1 food in invitory', 'eat successful'
#[test]
fn sat_hunger_test_ff() {
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
    blackboard.insert("food".to_owned(), Variable::Chit("Veggie".into()));

    // set up  the dummy values
    let find_in_inventory = vec![];
    let find_nearest = vec![house];
    let get_entities = vec![];
    let get_energy = vec![4];

    let get_location = vec![(6, 1), (6, 2)];
    let get_hp = vec![];
    let get_is_inventory_ge = vec![false, false];
    let running = vec![true, true, true];
    // done setting up dummy values

    let task_db = get_hermit_behavior_task();

    task_testing_harness(
        "sat_hunger",
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
