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

    let bt = get_hermit_behavior_task();
    logy!(
        "debug",
        "\n\n\n{:?}\n\n\n",
        bt.get("sat_hunger@2@1@1@1").unwrap()
    );
    let mut cpu = CPU::load("hermit".to_owned());
    loop {
        logy!(
            "debug",
            "\n\nexecuting {:?}\n stack: {:?}",
            cpu.pc,
            cpu.stack
        );
        match cpu.step(&bt, &mut blackboard) {
            Ok(status) => match status {
                Status::FindInInventory { item_class: _ } =>{
                    todo!()
                }
                Status::Success => {
                    logy!("trace", "hermit ai succeeded\n{cpu:?}");
                    break
                },
                Status::Failure => todo!(),
                Status::FindNearest { ../*x, y, item_class*/ } => todo!(),
                Status::GetEnergy(_entity) => {
                    logy!("trace", "giving dummy value for GetEnergy on {_entity}");
                    cpu.stack.push(StackItem::some(StackItem::Int(5)));
                },
                Status::GetLocation(_entity) => todo!(),
                Status::GetHp(_entity) => todo!(),
                Status::GetIsInventoryGE { agent: _agent, item_class: _item_class, amount: _amount } => {
                    logy!("trace", "giving dummy value for if {_agent} has GE {_amount} of {_item_class:?}");
                    cpu.stack.push(StackItem::success());
                },
                Status::GetEntities { ../*min_x, min_y, max_x, max_y*/ } => todo!(),
                Status::RemoveEntitiesOfType(_item) => todo!(),
                Status::RetainEntitiesOfType(_item) => todo!(),
                Status::Running(_inpulse_id) => todo!(),
                Status::None => (),
            },
            Err(err) => panic!("{err}"),
        }
    }
}
