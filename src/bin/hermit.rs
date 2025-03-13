use std::collections::HashMap;

use ethnolib::sandbox::{ai::{get_hermit_behavior_task, Blackboard, BlackboardValue, StackItem, Status, Variable, CPU}, EntityId, Item};
use qol::{logy, pout, Vecna};
type Player = Status;

/*
enum Player{
    FindInInventory { item_class},

}
*/
fn main(){
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
    main2(
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
fn main2(
    find_in_inventory: Vec<EntityId>,
    find_nearest: Vec<EntityId>,
    get_entities: Vec<StackItem>,
    get_energy: Vec<i32>,
    get_location: Vec<(i32,i32)>,
    get_hp: Vec<i32>,
    get_is_inventory_ge: Vec<bool>,
    running: Vec<bool>,
    mut blackboard: Blackboard<String, BlackboardValue>,
    item_types: HashMap<bevy::ecs::entity::Entity, Item>,
){
    let mut prayers = Vec::<(usize, Player)>::new();

    // These are for the dummy values
    let mut find_in_inventory_idx = 0;
    let mut find_nearest_idx = 0;
    let mut get_entities_idx = 0;
    let mut get_energy_idx = 0;
    let mut get_location_idx = 0;
    let mut get_hp_idx = 0;
    let mut get_is_inventory_ge_idx = 0;

    //
    let mut running_idx = 0;
    let mut remove_entities_of_type_idx = 0;
    let mut retain_entities_of_type_idx= 0;
    // end these ara for the dummy values

    let task_db = get_hermit_behavior_task();
    let mut cpu = CPU::load("hermit".to_owned());
    
    loop {
        pout!(
            "\nexecuting {}\nstack: {}\nreturn: {:?}",
            if let Some((a,b)) = &cpu.pc {format!("{a}:{b}")} else{ "None".to_owned()},
            Vecna::from(&cpu.stack),
            cpu.return_stack,
            
        );
        match cpu.step(&task_db, &mut blackboard) {
            Ok(status) => match status {
                Status::FindInInventory { item_class } =>{
                    prayers.push((find_in_inventory_idx, Player::FindInInventory { item_class}));

                    let new_stack_item = StackItem::some(
                        StackItem::EntityId(
                            find_in_inventory[find_in_inventory_idx].clone()
                        )
                    );
                    find_in_inventory_idx = (find_in_inventory_idx + 1) & find_in_inventory.len();
                    cpu.stack.push(new_stack_item);
                }
                Status::Success => {
                    /*
                    logy!("log", "hermit ai succeeded\n\n\n-----\n{cpu:#?}\n-----");
                    break
                    */
                },
                Status::Failure => (),
                Status::FindNearest { x, y, item_class} => {
                    logy!("log", "giving dummy value for nearest {item_class:?} to [{x}:{y}]");
                    prayers.push((find_nearest_idx, Player::FindNearest { x, y, item_class}));
                    let new_stack_item = StackItem::some(StackItem::EntityId(find_nearest[find_nearest_idx % find_nearest.len()].clone()));
                    find_nearest_idx += 1;
                    cpu.stack.push(new_stack_item);
                },
                Status::GetEnergy(entity) => {
                    logy!("log", "giving dummy value for GetEnergy on {entity}");
                    prayers.push((get_energy_idx, Player::GetEnergy(entity)));

                    let new_stack_item = StackItem::some(StackItem::Int(get_energy[get_energy_idx % get_energy.len()].clone()));
                    get_energy_idx += 1;
                    cpu.stack.push(new_stack_item);
                },
                Status::GetLocation(entity) => {
                    logy!("log", "giving dummy value for location of {entity}");
                    prayers.push((get_location_idx, Player::GetLocation(entity)));

                    let loc = get_location[get_location_idx % get_location.len()];
                    let new_stack_item = StackItem::some(StackItem::Coord { x:loc.0, y:loc.1 });
                    get_location_idx += 1;
                    cpu.stack.push(new_stack_item);
                },
                Status::GetHp(entity) => {
                    logy!("log", "giving dummy value for GetHp on {entity}");
                    prayers.push((get_hp_idx, Player::GetHp(entity)));

                    let new_stack_item = StackItem::some(StackItem::Int(get_hp[get_hp_idx % get_hp.len()].clone()));
                    get_hp_idx += 1;
                    cpu.stack.push(new_stack_item);
                },
                Status::GetIsInventoryGE { agent, item_class, amount } => {
                    logy!("log", "giving dummy value for if {agent} has GE {amount} of {item_class:?}");
                    prayers.push((get_is_inventory_ge_idx, Player::GetIsInventoryGE { agent, item_class, amount }));

                    let new_stack_item = if get_is_inventory_ge[get_is_inventory_ge_idx % get_is_inventory_ge.len()] {StackItem::success()} else {StackItem::failure()};
                    get_is_inventory_ge_idx += 1;
                    cpu.stack.push(new_stack_item);

                },
                Status::GetEntities { min_x, min_y, max_x, max_y } => {
                    logy!("log", "giving dummy value for entities in [{min_x} : {min_y}] to [{max_x} : {max_y}]");
                    prayers.push((get_entities_idx, Player::GetEntities { min_x, min_y, max_x, max_y }));

                    let new_stack_item = get_entities[get_entities_idx % get_entities.len()].clone();
                    get_entities_idx += 1;
                    cpu.stack.push(new_stack_item);
                },
                Status::RemoveEntitiesOfType(item) => {
                    if let Some(StackItem::Table(ref mut inner))= cpu.stack.last_mut() {
                        inner.map.borrow_mut().retain(
                            |_k, v|{
                                let StackItem::EntityId(id) = v else {
                                    return true
                                };
                                let Some(tyep) = item_types.get(id) else {
                                    return true
                                };
                                tyep != &item
                            }
                        );
                    }
                    prayers.push((remove_entities_of_type_idx, Player::RemoveEntitiesOfType(item)));
                    remove_entities_of_type_idx += 1;
                },
                Status::RetainEntitiesOfType(item) => {
                    if let Some(StackItem::Table(ref mut inner))= cpu.stack.last_mut() {
                        inner.map.borrow_mut().retain(
                            |_k, v|{
                                let StackItem::EntityId(id) = v else {
                                    return false
                                };
                                let Some(tyep) = item_types.get(id) else {
                                    return false
                                };
                                tyep == &item
                            }
                        );
                    }
                    prayers.push((retain_entities_of_type_idx, Player::RetainEntitiesOfType(item)));
                    retain_entities_of_type_idx += 1;
                },
                Status::Running(inpulse_id) => {
                    logy!("log", "doing dummy inpule {running_idx}. : {inpulse_id:?}");
                    match &inpulse_id {
                        ethnolib::sandbox::ai::InpulseId::EatClass(_) =>(),
                        ethnolib::sandbox::ai::InpulseId::Use |
                        ethnolib::sandbox::ai::InpulseId::Take |
                        ethnolib::sandbox::ai::InpulseId::GoTo => {
                            cpu.stack.pop();
                        }
                        _ => panic!("unhandled inpulse")
                    }
                    prayers.push((running_idx, Player::Running(inpulse_id)));
                    
                    

                    let new_stack_item = if running[running_idx % running.len()] {StackItem::success()} else {StackItem::failure()};
                    running_idx += 1;
                    cpu.stack.push(new_stack_item);

                },
                Status::None => (),
            },
            Err(err) => {
                for x in prayers {
                    pout!("{x:?}");
                };
                panic!("hermitAI gave error: {err}")
            },
        }
    }
}