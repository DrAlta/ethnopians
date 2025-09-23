use std::collections::BTreeSet;

use crate::{
    bvh::{calculate_morton_code, create_subtree, intersect, MortenCode, NodeType},
    types::AARect,
    Number,
};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Node<Id: std::fmt::Debug + Clone + PartialEq + Eq + PartialOrd + Ord + std::hash::Hash> {
    pub min_x: Number,
    pub min_y: Number,
    pub max_x: Number,
    pub max_y: Number,

    pub node_type: NodeType<Id>,
}

impl<'a, Id: std::fmt::Debug + Clone + PartialEq + Eq + PartialOrd + Ord + std::hash::Hash>
    Node<Id>
{
    pub fn create_tree<I, F>(list: I, get_aa_rect: &'a F) -> Result<Self, String>
    where
        I: IntoIterator<Item = Id> + Clone,
        F: Fn(&Id) -> Option<AARect>,
    {
        let mut iter1 = list.clone().into_iter();

        let Some(first_id) = iter1.next() else {
            return Err("No items to build tree from".to_owned());
        };
        let Some(AARect {
            min_x,
            min_y,
            width,
            height,
        }) = get_aa_rect(&first_id)
        else {
            return Err(format!(
                "Couldn't find world size:couldn't get AArect for {first_id:?}"
            ));
        };
        let mut world_min_x = min_x;
        let mut world_min_y = min_y;
        let mut world_max_x = min_x + width;
        let mut world_max_y = min_y + height;

        for id in iter1 {
            let Some(AARect {
                min_x,
                min_y,
                width,
                height,
            }) = get_aa_rect(&id)
            else {
                return Err(format!(
                    "Couldn't find world size::couldn't get AArech for {id:?}"
                ));
            };
            world_min_x = world_min_x.min(min_x);
            world_min_y = world_min_y.min(min_y);
            world_max_x = world_max_x.max(min_x + width);
            world_max_y = world_max_y.max(min_y + height);
        }
        let mut acc: Vec<(Id, MortenCode)> = Vec::new();
        for id in list {
            let Some(AARect {
                min_x,
                min_y,
                width,
                height,
            }) = get_aa_rect(&id)
            else {
                return Err(format!(
                    "Couldn't build morten codes:couldn't get AArech for {id:?}"
                ));
            };
            let x = min_x + (width * Number::HALF);
            let y = min_y + (height * Number::HALF);
            let morten = calculate_morton_code(
                &x,
                &y,
                &world_min_x,
                &world_min_y,
                &world_max_x,
                &world_max_y,
            );
            acc.push((id, morten));
        }
        Self::create_tree_from_morten(acc, get_aa_rect)
    }
    pub fn create_tree_from_morten<I, F>(list: I, get_aa_rect: &'a F) -> Result<Self, String>
    where
        I: IntoIterator<Item = (Id, MortenCode)>,
        F: Fn(&Id) -> Option<AARect>,
    {
        let mut sorted_by_morten: Vec<(Id, MortenCode)> = list.into_iter().collect();
        sorted_by_morten
            .sort_by(|(_, morten_code_a), (_, morten_code_b)| morten_code_a.cmp(morten_code_b));

        let end = sorted_by_morten.len() - 1;

        create_subtree(&sorted_by_morten, 0, end, get_aa_rect)
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
            &self.min_x,
            &self.min_y,
            &self.max_x,
            &self.max_y,
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
