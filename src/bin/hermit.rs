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
            "\nexecuting {}\nstack: {}",
            if let Some((a,b)) = &cpu.pc {format!("{a}:{b}")} else{ "None".to_owned()},
            Vecna::from(&cpu.stack)
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
                Status::FindNearest { ../*x, y, item_class*/ } => todo!(),
                Status::GetEnergy(_entity) => {
                    logy!("log", "giving dummy value for GetEnergy on {_entity}");
                    cpu.stack.push(StackItem::some(StackItem::Int(5)));
                },
                Status::GetLocation(_entity) => todo!(),
                Status::GetHp(_entity) => todo!(),
                Status::GetIsInventoryGE { agent: _agent, item_class: _item_class, amount: _amount } => {
                    logy!("log", "giving dummy value for if {_agent} has GE {_amount} of {_item_class:?}");
                    cpu.stack.push(StackItem::success());
                },
                Status::GetEntities { ../*min_x, min_y, max_x, max_y*/ } => todo!(),
                Status::RemoveEntitiesOfType(_item) => todo!(),
                Status::RetainEntitiesOfType(_item) => todo!(),
                Status::Running(inpulse_id) => {
                    match inpulse_id {
                        /*
                        ethnolib::sandbox::ai::InpulseId::Act1 => todo!(),
                        ethnolib::sandbox::ai::InpulseId::Act2 => todo!(),
                        ethnolib::sandbox::ai::InpulseId::Act3 => todo!(),
                        ethnolib::sandbox::ai::InpulseId::GoTo => todo!(),
                        ethnolib::sandbox::ai::InpulseId::Plant => todo!(),
                        ethnolib::sandbox::ai::InpulseId::Take => todo!(),
                        ethnolib::sandbox::ai::InpulseId::Use => todo!(),
                        */
                        //ethnolib::sandbox::ai::InpulseId::EatClass(_) => todo!(),
                        _ => {
                            cpu.stack.push(StackItem::String("Success".to_owned()));
                        }
                    }
                },
                Status::None => (),
            },
            Err(err) => panic!("hermitAI gave error: {err}"),
        }
    }
}