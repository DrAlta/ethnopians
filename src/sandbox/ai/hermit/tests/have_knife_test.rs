use std::{collections::BTreeMap, sync::Arc};

use crate::sandbox::{
    ai::{
        get_hermit_behavior_task, task_testing_harness, Blackboard, BlackboardValue, StackItem,
        Variable,
    },
    EntityId, Item,
};

#[test]
fn have_knife_test() {
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
    let find_in_inventory = vec![];
    let find_nearest = vec![house];
    let get_entities = vec![];
    let get_energy = vec![];

    let get_location = vec![(6, 1), (6, 2)];
    let get_hp = vec![];
    let get_is_inventory_ge = vec![false, true, false];
    let running = vec![true, true];
    // end seting up dummy values

    let task_db = get_hermit_behavior_task();

    task_testing_harness(
        "have_knife",
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
