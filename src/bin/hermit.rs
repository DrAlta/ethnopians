use std::collections::HashMap;


use ethnolib::sandbox::{ai::{get_hermit_behavior_task, task_testing_harness, Blackboard, BlackboardValue, StackItem, Variable}, EntityId, Item};

/*
enum Prayer{
    FindInInventory { item_class},

}
*/
fn main(){
    sat_hunger_test_ff()
}
// 'got 4 for energy', "energy not greater than 5", 'not greater than or equal than 1 food in invitory', 'eat successful'
fn sat_hunger_test_ff(){
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
        "food".to_owned(),
        Variable::Chit(BlackboardValue::String("Veggie".to_owned())),
    );

    // set up  the dummy values
    let find_in_inventory = vec![];
    let find_nearest = vec![house];
    let get_entities = vec![
        [
            (StackItem::Int(0), StackItem::EntityId(garden_1)),
            (StackItem::Int(1), StackItem::EntityId(garden_2)),
            (StackItem::Int(2), StackItem::EntityId(garden_3)),
            (StackItem::Int(3), StackItem::EntityId(garden_4)),
        ].try_into().unwrap()
    ];
    let get_energy = vec![4];

    let get_location = vec![
        (6,9),
    ];
    let get_hp= vec![4];
    let get_is_inventory_ge = vec![false];
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
#[allow(dead_code)]
fn foofoo(){
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
        "food".to_owned(),
        Variable::Chit(BlackboardValue::String("Veggie".to_owned())),
    );
    let find_in_inventory = vec![];
    let find_nearest = vec![house];
    let get_entities = vec![
        [
            (StackItem::Int(0), StackItem::EntityId(garden_1)),
            (StackItem::Int(1), StackItem::EntityId(garden_2)),
            (StackItem::Int(2), StackItem::EntityId(garden_3)),
            (StackItem::Int(3), StackItem::EntityId(garden_4)),
        ].try_into().unwrap()
    ];
    let get_energy = vec![7];

    let get_location = vec![
        (6,9),
    ];
    let get_hp= vec![4];
    let get_is_inventory_ge = vec![true];
    let running = vec![true];

    let task_db = get_hermit_behavior_task();

    task_testing_harness(
        "hermit",
        task_db,
        Vec::new(),
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
