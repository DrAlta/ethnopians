use std::collections::HashMap;

use crate::{
    broadphase::bvh::{MortenCode, Node, NodeType},
    types::AARect,
};

pub fn create_subtree<Id: std::hash::Hash>(
    sorted_list: &Vec<(Id, MortenCode)>,
    begin: usize,
    end: usize,
    entities: &HashMap<Id, AARect>,
) -> Result<Node<Id>, String>
where
    Id: std::fmt::Debug + Clone + PartialEq + Eq + PartialOrd + Ord,
{
    if begin == end {
        let id = sorted_list[begin].0.clone();
        let Some(AARect {
            min_x,
            min_y,
            width,
            height,
        }) = entities.get(&id).cloned()
        else {
            return Err(format!("faild fo find AArect for {id:?}"));
        };
        return Ok(Node {
            min_x,
            min_y,
            max_x: min_x + width,
            max_y: min_y + height,
            node_type: NodeType::Leaf(id),
        });
    } else {
        let m = (begin + end) / 2;
        let left = Box::new(
            create_subtree(sorted_list, begin, m, entities)
                .map_err(|err| format!("Could biuld left side: {err}"))?,
        );
        let right = Box::new(
            create_subtree(sorted_list, m + 1, end, entities)
                .map_err(|err| format!("Could biuld right side: {err}"))?,
        );

        // Update node's AABB to encompass children's AABBs
        let min_x = left.min_x.min(right.min_x);
        let min_y = left.min_y.min(right.min_y);

        let max_x = left.max_x.min(right.max_x);
        let max_y = left.max_y.min(right.max_y);

        return Ok(Node {
            min_x,
            min_y,
            max_x,
            max_y,
            node_type: NodeType::Branch { left, right },
        });
    }
}
