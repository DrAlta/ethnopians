mod expand_bits;
use std::collections::BTreeSet;

pub use expand_bits::expand_bits;

use crate::{types::AARect, Number};

pub type MortenCode = u32;

pub fn calculate_morton_code(x: &Number, y: &Number, min_x:& Number, min_y: &Number, max_x: &Number, max_y: &Number) -> MortenCode {
    let x2 = ((x - min_x) / (max_x - min_x) * 2_f32.powi(16_i32)).floor() as u16;
    let y2 = ((y - min_y) / (max_y - min_y) * 2_f32.powi(16_i32)).floor() as u16;

    expand_bits(x2) | (expand_bits(y2) << 1)
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum NodeType<Id: std::fmt::Debug + Clone + PartialEq + Eq + PartialOrd + Ord> {
    Leaf(Id),
    Branch{
        left: Box<Node<Id>>, 
        right: Box<Node<Id>>,
    },
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Node<Id: std::fmt::Debug + Clone + PartialEq + Eq + PartialOrd + Ord> {
    pub min_x: Number,
    pub min_y: Number,
    pub max_x: Number,
    pub max_y: Number,

    pub node_type: NodeType<Id>,
}

impl<'a, Id: std::fmt::Debug + Clone + PartialEq + Eq + PartialOrd + Ord> Node<Id> {
    pub fn create_tree<I, F>(
        list: I,
        get_aa_rect: &'a F,
    ) -> Result<Self, String>
    where 
        I: IntoIterator<Item = Id> + Clone,
        F: Fn(&Id) -> Option<AARect>,
    {
        let mut iter1 = list.clone().into_iter();

        let Some(first_id) = iter1.next() else {
            return Err("No items to build tree from".to_owned())
        };
        let Some(AARect { min_x, min_y, width, height }) = get_aa_rect(&first_id) else {
            return Err(format!("Couldn't find world size:couldn't get AArect for {first_id:?}"))
        };
        let mut world_min_x = min_x;
        let mut world_min_y = min_y;
        let mut world_max_x = min_x + width;
        let mut world_max_y = min_y + height;

        for id in iter1 {
            let Some(AARect { min_x, min_y, width, height }) = get_aa_rect(&id) else {
                return Err(format!("Couldn't find world size::couldn't get AArech for {id:?}"))
            };
            world_min_x = world_min_x.min(min_x);
            world_min_y = world_min_y.min(min_y);
            world_max_x = world_max_x.max(min_x + width);
            world_max_y = world_max_y.max(min_y + height);
        }
        let mut acc: Vec::<(Id, MortenCode)> = Vec::new();
        for id in list {
            let Some(AARect { min_x, min_y, width, height }) = get_aa_rect(&id) else {
                return Err(format!("Couldn't build morten codes:couldn't get AArech for {id:?}"))
            };
            let x = min_x + (width * Number::HALF);
            let y = min_y + (height * Number::HALF);
            let morten = calculate_morton_code(&x, &y, &world_min_x, &world_min_y, &world_max_x, &world_max_y);
            acc.push((id, morten));
        }
        Self::create_tree_from_morten(acc, get_aa_rect)
    }
    pub fn create_tree_from_morten<I, F>(
        list: I,
        get_aa_rect: &'a F,
    ) -> Result<Self, String>
    where 
        I: IntoIterator<Item = (Id, MortenCode)>,
        F: Fn(&Id) -> Option<AARect>,
    {
        let mut sorted_by_morten : Vec<(Id, MortenCode)> = list
            .into_iter()
            .collect();
        sorted_by_morten.sort_by(|(_, morten_code_a), (_, morten_code_b)| {
            morten_code_a.cmp(morten_code_b)
        });

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
            &self.max_y
        ) {
            return BTreeSet::new()
        }

        match &self.node_type {
            NodeType::Leaf(id) => return BTreeSet::from([id.clone()]),
            NodeType::Branch { left, right } => {
                let left = left.qurry(min_x, min_y, max_x, max_y);
                let mut ret = right.qurry(min_x, min_y, max_x, max_y);
                ret.extend(left);
                return ret
            },
        }
    }
}

pub fn create_subtree<'a, Id, F>(
    sorted_list: &Vec<(Id, MortenCode)>,
    begin: usize, 
    end: usize,
    get_aa_rect: &'a F,
) -> Result<Node<Id>, String>
where 
    Id: std::fmt::Debug + Clone + PartialEq + Eq + PartialOrd + Ord,
    F: Fn(&Id) -> Option<AARect>,
{
    if begin == end {
        let id = sorted_list[begin].0.clone();
        let Some(AARect { min_x, min_y, width, height }) = get_aa_rect(&id) else {
            return Err(format!("faild fo find AArect for {id:?}"))
        };
        return Ok(Node { 
            min_x, 
            min_y, 
            max_x: min_x + width, 
            max_y: min_y + height, 
            node_type:NodeType::Leaf(id) 
        })
    } else {
        let m = (begin + end ) / 2;
        let left = Box::new(create_subtree(sorted_list, begin, m -1, get_aa_rect)
            .map_err(
                |err| 
                format!("Could biuld left side: {err}")
            )?);
        let right = Box::new(create_subtree(sorted_list, m + 1, end, get_aa_rect)
            .map_err(
                |err| 
                format!("Could biuld right side: {err}")
            )?);

            // Update node's AABB to encompass children's AABBs
            let min_x = left.min_x.min(right.min_x);
            let min_y = left.min_y.min(right.min_y);
            
            let max_x = left.max_x.min(right.max_x);
            let max_y = left.max_y.min(right.max_y);

            return Ok(Node{ min_x, min_y, max_x, max_y, node_type: NodeType::Branch { left, right} })
    }
}

fn intersect(
    a_min_x: &Number,
    a_min_y: &Number,
    a_max_x: &Number,
    a_max_y: &Number,
    b_min_x: &Number,
    b_min_y: &Number,
    b_max_x: &Number,
    b_max_y: &Number,
) -> bool {
    a_max_y > b_min_y
        && b_max_y > a_min_y
        && a_max_x > b_min_x
        && b_max_x  > a_min_x
}