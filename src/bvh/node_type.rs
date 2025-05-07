use crate::bvh::Node;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum NodeType<Id: std::fmt::Debug + Clone + PartialEq + Eq + PartialOrd + Ord + std::hash::Hash>
{
    Leaf(Id),
    Branch {
        left: Box<Node<Id>>,
        right: Box<Node<Id>>,
    },
}
