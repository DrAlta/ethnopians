use crate::ai::behavior_tree::Node;

mod task_master;
pub use task_master::TaskMaster;
mod tick;
mod walk_down;
pub use walk_down::walk_down;
mod walk_up;
pub use walk_up::walk_up;


type TickCount = u8;

type Path<'a>  = Vec<&'a Node>;
