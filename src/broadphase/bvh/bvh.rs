use std::collections::BTreeSet;

use crate::{Number, broadphase::{Broadphase, Node, bvh::NodeType}, types::AARect};

pub struct BVH{
    node_maybe: Option<Node<usize>>,
    lookup: Vec<Vec<bool>>,
}
impl Broadphase<usize> for BVH {
    fn new<I: Iterator<Item = AARect>>(entities: I) -> Self {
        let mut ret = Self { node_maybe: None, lookup: Vec::new()};
        for c in entities {
            ret.insert(c);
        }
        ret
    }

    fn insert(&mut self, aabb: AARect) -> usize {
        let id = self.lookup.len();
        match &mut self.node_maybe {
            Some(node) => {
                self.lookup.push(node.add(id, aabb, Vec::new()));
            },
            None => {
                self.node_maybe = Some(Node{bounds: aabb, node_type: NodeType::Leaf(id)});
                self.lookup.push(Vec::new());
            },
        }
        id
    }

    fn ready(&mut self) -> bool {
        true
    }

    fn qurry(
        &self,
        min_x: Number,
        min_y: Number,
        max_x: Number,
        max_y: Number,
    ) -> BTreeSet<usize> {
        let mut ret = BTreeSet::new();
        let aa_rect = AARect::from_min_max(min_x, min_y, max_x, max_y);
        match &self.node_maybe {
            Some(first) => {
                let mut todo = vec![first];
                while let Some(node) = todo.pop() {
                    if aa_rect.intersects(&node.bounds) {
                        match &node.node_type {
                            NodeType::Leaf(id) => {
                                ret.insert(id.clone());
                            },
                            NodeType::Branch { left, right } => {
                                todo.push(left);
                                todo.push(right);
                            },
                        }
                    }
                }
                ret
            },
            None => ret,
        }

    }

    fn get_entity(&self, k: &usize) -> Option<AARect> {
        let mut counter = 0;
        let path = self.lookup.get(k.clone())?;
        let mut node = self.node_maybe.as_ref()?;
        loop {
            match &node.node_type {
                NodeType::Leaf(id) => {
                    if id == k {
                        return Some(node.bounds.clone())
                    }
                },
                NodeType::Branch { left, right } => {
                    node = if path.get(counter)?.clone() {
                        left
                    } else {
                        right
                    };
                    counter += 1;
                },
            }
        }
    }
}