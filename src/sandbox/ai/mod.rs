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
use std::collections::BTreeMap;

mod cpu;
pub use cpu::{
    task_testing_harness, Blackboard, BlackboardValue, Instruction, Stack, StackItem, TableGet,
    TableInterior, Variable, CPU,
};
mod behavior_tree;
pub use behavior_tree::Corrent;
mod hermit;
pub use hermit::get_hermit_behavior_task;
mod inpulse_id;
pub use inpulse_id::InpulseId;
pub mod parser;
mod status;
pub use status::Status;

pub type BlackboardKey = String;
pub type ThreadName = String;
pub type ExecutionToken = ThreadName;
pub type ExecutionPointer = (ExecutionToken, usize);
pub type Thread = Vec<Instruction>;
pub type TreePool = BTreeMap<ThreadName, Thread>;
pub type ItemId = String;
