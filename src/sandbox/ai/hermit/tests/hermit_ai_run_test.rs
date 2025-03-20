use qol::logy;

use crate::sandbox::{
    ai::{get_hermit_behavior_task, Blackboard, BlackboardValue, StackItem, Status, Variable, CPU},
    EntityId,
};

#[test]
fn hermit_ai_run_test() {
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
        logy!(
            "trace-tests",
            "\nexecuting {:?}\nstack: {:?}\n",
            cpu.pc,
            cpu.stack
        );
        match cpu.step(&task_db, &mut blackboard) {
            Ok(status) => match status {
                Status::UseOn(_a, _b) =>{
                    todo!()
                }
                Status::FindInInventory { item_class: _ } =>{
                    todo!()
                }
                Status::Success => {
                    logy!("trace-tests", "hermit ai succeeded\n{cpu:?}");
                    break
                },
                Status::Failure => todo!(),
                Status::FindNearest { ../*x, y, item_class*/ } => todo!(),
                Status::GetEnergy(_entity) => {
                    logy!("trace-tests", "giving dummy value for GetEnergy on {_entity}");
                    cpu.stack.push(StackItem::some(StackItem::Int(5)));
                },
                Status::GetLocation(_entity) => todo!(),
                Status::GetHp(_entity) => todo!(),
                Status::GetIsInventoryGE { agent: _agent, item_class: _item_class, amount: _amount } => {
                    logy!("trace-tests", "giving dummy value for if {_agent} has GE {_amount} of {_item_class:?}");
                    cpu.stack.push(StackItem::success());
                },
                Status::GetEntities { ../*min_x, min_y, max_x, max_y*/ } => todo!(),
                Status::RemoveEntitiesOfType(_item) => todo!(),
                Status::RetainEntitiesOfType(_item) => todo!(),
                Status::Running(_inpulse_id) => todo!(),
                Status::None => (),
            },
            Err(err) => panic!("hermitAI gave error: {err}"),
        }
    }
}
