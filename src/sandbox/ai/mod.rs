//! for the test aI just have it plant vegtibles in a field and harvest them ehen they are mature then replant them
//! if out od seed find neared plant to collect seeds from
//! i thing a veg can be split into 3 seeds
//! useing hands on a plant produces vegs and consumes the plant
//! use an knife of a veg produces 3 seeds and consumes the veg
//!
//! use a stone on stone produces a knife and consomes one stone
//!
//! useinga knife on stick or visvera produces a axe and consumes the knife and stick
//!
//! knife has higher DPS than axe but shorter range

mod cpu;
use std::collections::HashMap;

pub use cpu::CPU;
mod behavior_tree;
pub use behavior_tree::Corrent;
mod blackboard;
pub use blackboard::{Blackboard, Variable};
mod blackboard_value;
pub use blackboard_value::BlackboardValue;
mod hermit;
pub use hermit::get_hermit_behavoir_tree;
mod inpulse_id;
pub use inpulse_id::InpulseId;
mod instruction;
pub use instruction::Instruction;
pub mod parser;
mod stack_item;
pub use stack_item::StackItem;
mod status;
pub use status::Status;

pub type BlackboardKey = String;
pub type ThreadName = String;
pub type ExecutionToken = ThreadName;
pub type ExecutionPointer = (ExecutionToken, usize);
pub type Thread = Vec<Instruction>;
pub type TreePool = HashMap<ThreadName, Thread>;
pub type ItemId = String;

#[cfg(test)]
mod tests {
    use qol::logy;

    use crate::sandbox::{
        ai::{get_hermit_behavoir_tree, Blackboard, BlackboardValue, StackItem, Variable, CPU},
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

        let bt = get_hermit_behavoir_tree();
        logy!(
            "debug",
            "\n\n\n{:?}\n\n\n",
            bt.get("sat_hunger_2_1_1_1").unwrap()
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
                    super::Status::Success => {
                        logy!("trace", "hermit ai succeeded\n{cpu:?}");
                        break
                    },
                    super::Status::Failure => todo!(),
                    super::Status::FindNearest { ../*x, y, item_class*/ } => todo!(),
                    super::Status::GetEnergy(entity) => {
                        logy!("trace", "giving dummy value for GetEnergy on {entity}");
                        cpu.stack.push(StackItem::some(StackItem::Int(5)));
                    },
                    super::Status::GetLocation(_entity) => todo!(),
                    super::Status::GetHp(_entity) => todo!(),
                    super::Status::GetIsInventoryGE { agent, item_class, amount } => {
                        logy!("trace", "giving dummy value for if {agent} has GE {amount} of {item_class:?}");
                        cpu.stack.push(StackItem::Success);
                    },
                    super::Status::GetEntities { ../*min_x, min_y, max_x, max_y*/ } => todo!(),
                    super::Status::RemoveEntitiesOfType(_item) => todo!(),
                    super::Status::Running(_inpulse_id) => todo!(),
                    super::Status::None => (),
                },
                Err(err) => panic!("{err}"),
            }
        }
    }
}
