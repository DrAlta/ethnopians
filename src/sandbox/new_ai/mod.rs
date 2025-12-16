mod blackboard;
pub use blackboard::{Blackboard, BlackboardValue, Variable};
pub mod behavior_tree;
pub mod forth;
//pub mod forth_parser;
mod prayer;
pub use prayer::Prayer;
mod status;
pub use status::Status;
mod task_master;
pub use task_master::{TaskMaster, TastMasterRet};


type InpulseId = usize;

pub type BlackboardKey = String;
