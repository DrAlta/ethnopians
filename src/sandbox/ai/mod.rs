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

    use crate::sandbox::
        ai::{get_hermit_behavoir_tree, Blackboard, CPU};

    #[test]
    fn hermit_ai_run_test() {
        let mut blackboard = Blackboard::new();

        let bt = get_hermit_behavoir_tree();
        let mut cpu = CPU::load("hermit".to_owned());
        loop {
            logy!("debug", "{:?}\n executing {:?}", cpu.stack, cpu.pc);
            match cpu.step(&bt, &mut blackboard) {
                Ok(status) => match status {
                    super::Status::Success => todo!(),
                    super::Status::Failure => todo!(),
                    super::Status::FindNearest { ../*x, y, item_class*/ } => todo!(),
                    super::Status::GetEnergy(_entity) => todo!(),
                    super::Status::GetLocation(_entity) => todo!(),
                    super::Status::GetHp(_entity) => todo!(),
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
