use crate::ai::behavior_tree::Node;

use crate::ai::task_master::{sub_system_state::Branch, task_master::Path};

pub fn walk_down<'a, 'b>(tree: &'a Node, execution_limb: &'b Vec::<Branch>) -> Result<(Path<'a>, &'a Node), String>{
let mut path = Vec::new();
    let mut this_node = tree;
    for Branch{ child_index, .. } in execution_limb {
        path.push(this_node);
        this_node = match this_node.get_child(*child_index){
            Ok(c) => c,
            Err(err) => {
                return Err(format!("[{}:{}] while walking execution_limb{tree:?}:{path:?} got err{err:?}", file!(), line!()))
            }
        };
    }
    Ok((path, this_node))
}
