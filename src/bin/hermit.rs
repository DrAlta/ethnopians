use ethnolib::sandbox::{ai::{get_hermit_behavior_task, Blackboard, BlackboardValue, StackItem, Status, Variable, CPU}, EntityId};
use qol::{logy, pout, Vecna};

fn main(){
    main2()
}
fn main2(){
    let mut blackboard = Blackboard::new();
    blackboard.insert(
        "self".to_owned(),
        Variable::Chit(BlackboardValue::EntityId(EntityId::from_raw(0))),
    );
    blackboard.insert(
        "food".to_owned(),
        Variable::Chit(BlackboardValue::String("Veggie".to_owned())),
    );

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
                Status::FindInInventory { item_class: _ } =>{
                    todo!()
                }
                Status::Success => {
                    /*
                    logy!("log", "hermit ai succeeded\n\n\n-----\n{cpu:#?}\n-----");
                    break
                    */
                },
                Status::Failure => todo!(),
                Status::FindNearest { x: _x, y: _y, item_class: _item_class} => {
                    logy!("log", "giving dummy value for nearest {_item_class:?} to [{_x}:{_y}]");
                    cpu.stack.push(StackItem::some(StackItem::EntityId(EntityId::from_raw(4))));
                },
                Status::GetEnergy(_entity) => {
                    logy!("log", "giving dummy value for GetEnergy on {_entity}");
                    cpu.stack.push(StackItem::some(StackItem::Int(5)));
                },
                Status::GetLocation(_entity) => {
                    logy!("log", "giving dummy value for location of {_entity}");
                    cpu.stack.push(StackItem::some(StackItem::Coord { x: 6, y: 9 }));
                },
                Status::GetHp(_entity) => {
                    logy!("log", "giving dummy value for GetHp on {_entity}");
                    cpu.stack.push(StackItem::some(StackItem::Int(4)));
                },
                Status::GetIsInventoryGE { agent: _agent, item_class: _item_class, amount: _amount } => {
                    logy!("log", "giving dummy value for if {_agent} has GE {_amount} of {_item_class:?}");
                    cpu.stack.push(StackItem::success());
                },
                Status::GetEntities { min_x: _min_x, min_y: _min_y, max_x: _max_x, max_y: _max_y } => {
                    logy!("log", "giving dummy value for entities in [{_min_x} : {_min_y}] to [{_max_x} : {_max_y}]");
                    cpu.stack.push([
                        (StackItem::Int(0), StackItem::EntityId(EntityId::from_raw(50))),
                        (StackItem::Int(1), StackItem::EntityId(EntityId::from_raw(51))),
                        (StackItem::Int(2), StackItem::EntityId(EntityId::from_raw(52))),
                        (StackItem::Int(3), StackItem::EntityId(EntityId::from_raw(53))),
                    ].try_into().unwrap());
                },
                Status::RemoveEntitiesOfType(_item) => {
                    logy!("todo", "need to implement RemoveEntitiesOfType");
                },
                Status::RetainEntitiesOfType(_item) => {
                    logy!("todo", "need to implement RetainEntitiesOfType");
                },
                Status::Running(inpulse_id) => {
                    match inpulse_id {
                        /*
                        ethnolib::sandbox::ai::InpulseId::Act1 => todo!(),
                        ethnolib::sandbox::ai::InpulseId::Act2 => todo!(),
                        ethnolib::sandbox::ai::InpulseId::Act3 => todo!(),
                        */
                        ethnolib::sandbox::ai::InpulseId::Plant |
                        ethnolib::sandbox::ai::InpulseId::Take |
                        ethnolib::sandbox::ai::InpulseId::Use |
                        ethnolib::sandbox::ai::InpulseId::EatClass(_) |
                        ethnolib::sandbox::ai::InpulseId::GoTo => {
                            cpu.stack.pop();
                            cpu.stack.push(StackItem::success());
                        },
                        _ => {
                            cpu.stack.push(StackItem::success());
                        }
                    }
                },
                Status::None => (),
            },
            Err(err) => panic!("hermitAI gave error: {err}"),
        }
    }
}