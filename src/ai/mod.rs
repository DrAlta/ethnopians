mod blackboard;
pub use blackboard::{Blackboard, BlackboardValue, Variable};
pub mod behavior_tree;
pub mod forth;
//pub mod forth_parser;
mod prayer;
pub use prayer::Prayer;
mod status;
pub use status::Status;
#[cfg(feature = "taskmaster")]
mod task_master;
#[cfg(feature = "taskmaster")]
pub use task_master::{TaskMaster, TastMasterReport};


type InpulseId = usize;

pub type BlackboardKey = String;

#[cfg(test)]
pub mod test;