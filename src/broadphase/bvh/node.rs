use std::{
    collections::{BTreeSet, HashMap}, fmt::Debug, hash::Hash
};

use crate::{
    Number, broadphase::bvh::{MortenCode, NodeType, create_subtree, calculate_morton_code, intersect}, types::AARect
};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Node<Id>{//: std::fmt::Debug + Clone + PartialEq + Eq + PartialOrd + Ord + std::hash::Hash> {
    pub bounds: AARect,
    pub node_type: NodeType<Id>,
}
impl<Id: Clone> Node<Id> {
    pub fn add(&mut self, id: Id, aa_rect: AARect, mut path: Vec<bool>) -> Vec<bool> {
        let new_bounds = self.bounds.union(&aa_rect);
        let old_bounds = std::mem::replace(&mut self.bounds, new_bounds);

        match &mut self.node_type {
            NodeType::Leaf(old_id) => {

                let left_node = Node{ bounds: old_bounds, node_type: NodeType::Leaf(old_id.clone()) };
                let left = Box::new(left_node);

                let right_node = Node{ bounds: aa_rect, node_type: NodeType::Leaf(id) };
                let right = Box::new(right_node);

                self.node_type = NodeType::Branch { left, right };
                path.push(false);
                path
            },
            NodeType::Branch { left, right } => {
                if aa_rect.better_to_merg_with_a_than_b_ka(&left.bounds, &right.bounds) {
                    path.push(true);
                    left.add(id, aa_rect, path)
                } else {
                    path.push(false);
                    right.add(id, aa_rect, path)
                }

            },
        }
    }
} 

impl<Id: std::fmt::Debug + Clone + PartialEq + Eq + PartialOrd + Ord + std::hash::Hash> Node<Id> {
    pub fn create_tree(entities: &HashMap<Id, AARect>) -> Result<Self, String> {
        let mut iter1 = entities.iter().map(|(k, _)| k.clone());

        let Some(first_id) = iter1.next() else {
            return Err("No items to build tree from".to_owned());
        };
        let Some(aa_rect) = entities.get(&first_id).cloned()
        else {
            return Err(format!(
                "Couldn't find world size:couldn't get AArect for {first_id:?}"
            ));
        };
        let mut world_min_x = aa_rect.min_x();
        let mut world_min_y = aa_rect.min_y();
        let mut world_max_x = aa_rect.max_x();
        let mut world_max_y = aa_rect.max_y();

        for id in iter1 {
            let Some(aa_rect2) = entities.get(&id).cloned()
            else {
                return Err(format!(
                    "Couldn't find world size::couldn't get AArech for {id:?}"
                ));
            };
            world_min_x = world_min_x.min(aa_rect2.min_x());
            world_min_y = world_min_y.min(aa_rect2.min_y());
            world_max_x = world_max_x.max(aa_rect2.max_x());
            world_max_y = world_max_y.max(aa_rect2.max_y());
        }
        let mut acc: Vec<(Id, MortenCode)> = Vec::new();
        for (id, aa_ref) in entities {
            let aa_rect3 = aa_ref.clone();
            let x = aa_rect3.min_x() + (aa_rect3.width() * Number::HALF);
            let y = aa_rect3.min_y() + (aa_rect3.height() * Number::HALF);
            let morten = calculate_morton_code(
                &x,
                &y,
                &world_min_x,
                &world_min_y,
                &world_max_x,
                &world_max_y,
            );
            acc.push((id.clone(), morten));
        }
        Self::create_tree_from_morten(acc, entities)
    }
    pub fn create_tree_from_morten<'a, I>(
        list: I,
        entities: &HashMap<Id, AARect>,
    ) -> Result<Self, String>
    where
        I: IntoIterator<Item = (Id, MortenCode)>,
    {
        let mut sorted_by_morten: Vec<(Id, MortenCode)> = list.into_iter().collect();
        sorted_by_morten
            .sort_by(|(_, morten_code_a), (_, morten_code_b)| morten_code_a.cmp(morten_code_b));

        let end = sorted_by_morten.len() - 1;

        create_subtree(&sorted_by_morten, 0, end, entities)
    }
    pub fn qurry(
        &self,
        min_x: &Number,
        min_y: &Number,
        max_x: &Number,
        max_y: &Number,
    ) -> BTreeSet<Id> {
        if !intersect(
            min_x,
            min_y,
            max_x,
            max_y,
            &self.bounds.min_x(),
            &self.bounds.min_y(),
            &self.bounds.max_x(),
            &self.bounds.max_y(),
        ) {
            return BTreeSet::new();
        }

        match &self.node_type {
            NodeType::Leaf(id) => return BTreeSet::from([id.clone()]),
            NodeType::Branch { left, right } => {
                let left = left.qurry(min_x, min_y, max_x, max_y);
                let mut ret = right.qurry(min_x, min_y, max_x, max_y);
                ret.extend(left);
                return ret;
            }
        }
    }
}
