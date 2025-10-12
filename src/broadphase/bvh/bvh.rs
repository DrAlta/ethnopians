use std::collections::HashMap;

use crate::{broadphase::{Broadphase, Node, SpatialId}, types::AARect};


pub struct BVH<Id: std::fmt::Debug + Clone + PartialEq + Eq + PartialOrd + Ord + std::hash::Hash> {
    node_maybe: Option<Node<Id>>,
}

impl Broadphase for BVH<SpatialId> {
    fn new<'a, I: Iterator<Item = crate::types::AARect>>(entities: I) -> Self {
        let lookup: HashMap<SpatialId, AARect> = entities.enumerate().collect();
        let node_maybe = Node::create_tree(&lookup).ok();

        BVH { node_maybe }

    }

    fn insert(&mut self, aabb: crate::types::AARect) -> crate::broadphase::SpatialId {
        match self.node_maybe{
            Some(node) => {
                node.
            },
            None => {
                self.node_maybe = Node::create_tree(
                    &HashMap::from([(0,aabb)])
                ).ok();
                return 0
            },
        }
    }

    fn ready(&mut self) -> bool {
        todo!()
    }

    fn qurry(
        &self,
        min_x: crate::Number,
        min_y: crate::Number,
        max_x: crate::Number,
        max_y: crate::Number,
    ) -> std::collections::BTreeSet<crate::broadphase::SpatialId> {
        todo!()
    }

    fn get_entity(&self, k: &crate::broadphase::SpatialId) -> Option<crate::types::AARect> {
        todo!()
    }
}