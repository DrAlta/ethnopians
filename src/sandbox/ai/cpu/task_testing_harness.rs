use std::{collections::BTreeMap, sync::Arc};

use qol::{logy, pout, Vecna};

use crate::sandbox::{
    ai::{Blackboard, BlackboardValue, InpulseId, StackItem, Status, TreePool, CPU},
    EntityId, Item,
};

type Prayer = Status;

#[derive(Debug, Clone, Eq, PartialOrd, Ord)]
enum Info {
    Result,
    Standard,
}
impl std::cmp::PartialEq for Info {
    fn eq(&self, _other: &Self) -> bool {
        true
    }
}

pub fn task_testing_harness(
    task: &str,
    task_db: TreePool,
    what_final_stack_should_be: crate::sandbox::ai::Stack,
    find_in_inventory: Vec<EntityId>,
    find_nearest: Vec<EntityId>,
    get_entities: Vec<StackItem>,
    get_energy: Vec<i32>,
    get_location: Vec<(i32, i32)>,
    get_hp: Vec<i32>,
    get_is_inventory_ge: Vec<bool>,
    running: Vec<bool>,
    mut blackboard: Blackboard<String, BlackboardValue>,
    item_types: BTreeMap<bevy::ecs::entity::Entity, Item>,
) {
    let mut prayers = Vec::<(usize, Prayer)>::new();

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
    let mut retain_entities_of_type_idx = 0;
    // end these ara for the dummy values

    let mut cpu = CPU::load(task.to_owned());

    loop {
        if cpu.pc.is_none() {
            let rs: Vec<String> = cpu
                .return_stack
                .iter()
                .map(|(a, b)| format!("{a}:{b}"))
                .collect();
            pout!(
                "\n\n\nFinished executing {task}\nStack:{}\nReturn Stack:{}",
                Vecna::from(&cpu.stack),
                Vecna::from(&rs),
            );
            for x in prayers {
                pout!("{x:?}");
            }
            assert_eq!(find_in_inventory.len(), find_in_inventory_idx);
            assert_eq!(find_nearest.len(), find_nearest_idx);
            assert_eq!(get_entities.len(), get_entities_idx);
            assert_eq!(get_energy.len(), get_energy_idx);
            assert_eq!(get_location.len(), get_location_idx);
            assert_eq!(get_hp.len(), get_hp_idx);
            assert_eq!(get_is_inventory_ge.len(), get_is_inventory_ge_idx);
            assert_eq!(running.len(), running_idx);
            assert_eq!(
                (Info::Result, cpu.stack),
                (Info::Standard, what_final_stack_should_be)
            );
            return;
        }

        pout!(
            "\nexecuting {}\nstack: {}\nreturn: {:?}",
            if let Some((a, b)) = &cpu.pc {
                format!("{a}:{b}")
            } else {
                "None".to_owned()
            },
            Vecna::from(&cpu.stack),
            cpu.return_stack,
        );
        match cpu.step(&task_db, &mut blackboard) {
            Ok(status) => match status {
                Status::UseOn(_a, _b) => {
                    logy!("log", "givving Succes for using {_a} on {_b}");
                    cpu.stack.push(StackItem::success());
                }
                Status::FindInInventory { item_class } => {
                    prayers.push((
                        find_in_inventory_idx,
                        Prayer::FindInInventory { item_class },
                    ));

                    if find_in_inventory_idx >= find_in_inventory.len() {
                        panic!(
                            "only had {} items in find_in_inventory tried to access {}.",
                            find_in_inventory.len(),
                            find_in_inventory_idx + 1
                        );
                    };

                    let new_stack_item = StackItem::some(StackItem::EntityId(
                        find_in_inventory[find_in_inventory_idx].clone(),
                    ));
                    find_in_inventory_idx += 1;
                    cpu.stack.push(new_stack_item);
                }
                Status::Success => {
                    /*
                    logy!("log", "hermit ai succeeded\n\n\n-----\n{cpu:#?}\n-----");
                    break
                    */
                }
                Status::Failure => (),
                Status::FindNearest { x, y, item_class } => {
                    logy!(
                        "log",
                        "giving dummy value for nearest {item_class:?} to [{x}:{y}]"
                    );
                    prayers.push((find_nearest_idx, Prayer::FindNearest { x, y, item_class }));

                    if find_nearest_idx >= find_nearest.len() {
                        panic!(
                            "only had {} items in find_nearest tried to access {}.",
                            find_nearest.len(),
                            find_nearest_idx + 1
                        );
                    };

                    let new_stack_item = StackItem::some(StackItem::EntityId(
                        find_nearest[find_nearest_idx].clone(),
                    ));
                    find_nearest_idx += 1;
                    cpu.stack.push(new_stack_item);
                }
                Status::GetEnergy(entity) => {
                    logy!("log", "giving dummy value for GetEnergy on {entity}");
                    prayers.push((get_energy_idx, Prayer::GetEnergy(entity)));

                    if get_energy_idx >= get_energy.len() {
                        panic!(
                            "only had {} items in get_energy tried to access {}.",
                            get_energy.len(),
                            get_energy_idx + 1
                        );
                    };

                    let new_stack_item =
                        StackItem::some(StackItem::Int(get_energy[get_energy_idx].clone()));
                    get_energy_idx += 1;
                    cpu.stack.push(new_stack_item);
                }
                Status::GetLocation(entity) => {
                    logy!("log", "giving dummy value for location of {entity}");
                    prayers.push((get_location_idx, Prayer::GetLocation(entity)));

                    if get_location_idx >= get_location.len() {
                        panic!(
                            "only had {} items in get_location tried to access {}.",
                            get_location.len(),
                            get_location_idx + 1
                        );
                    };

                    let loc = get_location[get_location_idx];
                    let new_stack_item = StackItem::some(StackItem::Coord { x: loc.0, y: loc.1 });
                    get_location_idx += 1;
                    cpu.stack.push(new_stack_item);
                }
                Status::GetHp(entity) => {
                    logy!("log", "giving dummy value for GetHp on {entity}");
                    prayers.push((get_hp_idx, Prayer::GetHp(entity)));

                    if get_hp_idx >= get_hp.len() {
                        panic!(
                            "only had {} items in get_hp tried to access {}.",
                            get_hp.len(),
                            get_hp_idx + 1
                        );
                    };

                    let new_stack_item =
                        StackItem::some(StackItem::Int(get_hp[get_hp_idx].clone()));
                    get_hp_idx += 1;
                    cpu.stack.push(new_stack_item);
                }
                Status::GetIsInventoryGE {
                    agent,
                    item_class,
                    amount,
                } => {
                    logy!(
                        "log",
                        "giving dummy value for if {agent} has GE {amount} of {item_class:?}"
                    );
                    prayers.push((
                        get_is_inventory_ge_idx,
                        Prayer::GetIsInventoryGE {
                            agent,
                            item_class,
                            amount,
                        },
                    ));

                    if get_is_inventory_ge_idx >= get_is_inventory_ge.len() {
                        panic!(
                            "only had {} items in get_is_inventory_ge tried to access {}.",
                            get_is_inventory_ge.len(),
                            get_is_inventory_ge_idx + 1
                        );
                    };

                    let new_stack_item = if get_is_inventory_ge[get_is_inventory_ge_idx] {
                        StackItem::success()
                    } else {
                        StackItem::failure()
                    };
                    get_is_inventory_ge_idx += 1;
                    cpu.stack.push(new_stack_item);
                }
                Status::GetEntities {
                    min_x,
                    min_y,
                    max_x,
                    max_y,
                } => {
                    logy!("log", "giving dummy value for entities in [{min_x} : {min_y}] to [{max_x} : {max_y}]");
                    prayers.push((
                        get_entities_idx,
                        Prayer::GetEntities {
                            min_x,
                            min_y,
                            max_x,
                            max_y,
                        },
                    ));

                    if get_entities_idx >= get_entities.len() {
                        panic!(
                            "only had {} items in get_entities tried to access {}.",
                            get_entities.len(),
                            get_entities_idx + 1
                        );
                    };

                    let new_stack_item = get_entities[get_entities_idx].clone();
                    get_entities_idx += 1;
                    cpu.stack.push(new_stack_item);
                }
                Status::RemoveEntitiesOfType(item) => {
                    if let Some(StackItem::Table(ref mut inner)) = cpu.stack.last_mut() {
                        Arc::make_mut(inner).map.retain(|_k, v| {
                            let StackItem::EntityId(id) = v else {
                                return true;
                            };
                            let Some(tyep) = item_types.get(id) else {
                                return true;
                            };
                            tyep != &item
                        });
                    }
                    prayers.push((
                        remove_entities_of_type_idx,
                        Prayer::RemoveEntitiesOfType(item),
                    ));
                    remove_entities_of_type_idx += 1;
                }
                Status::RetainEntitiesOfType(item) => {
                    if let Some(StackItem::Table(ref mut inner)) = cpu.stack.last_mut() {
                        Arc::make_mut(inner).map.retain(|_k, v| {
                            let StackItem::EntityId(id) = v else {
                                return false;
                            };
                            let Some(tyep) = item_types.get(id) else {
                                return false;
                            };
                            tyep == &item
                        });
                    }
                    prayers.push((
                        retain_entities_of_type_idx,
                        Prayer::RetainEntitiesOfType(item),
                    ));
                    retain_entities_of_type_idx += 1;
                }
                Status::Running(inpulse_id) => {
                    logy!("log", "doing dummy inpule {running_idx}. : {inpulse_id:?}");
                    match &inpulse_id {
                        InpulseId::Act1
                        | InpulseId::Act2
                        | InpulseId::Act3
                        | InpulseId::EatClass(_) => (),

                        InpulseId::Use | InpulseId::Take | InpulseId::Plant | InpulseId::GoTo => {
                            cpu.stack.pop();
                        } //_ => panic!("unhandled inpulse")
                    }
                    prayers.push((running_idx, Prayer::Running(inpulse_id)));

                    if running_idx >= running.len() {
                        panic!(
                            "only had {} items in running tried to access {}.",
                            running.len(),
                            running_idx + 1
                        );
                    };

                    let new_stack_item = if running[running_idx] {
                        StackItem::success()
                    } else {
                        StackItem::failure()
                    };
                    running_idx += 1;
                    cpu.stack.push(new_stack_item);
                }
                Status::None => (),
            },
            Err(err) => {
                for x in prayers {
                    pout!("{x:?}");
                }
                panic!("hermitAI gave error: {err}")
            }
        }
    }
}
