use crate::bvh::Node;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum NodeType<Id: std::fmt::Debug + Clone + PartialEq + Eq + PartialOrd + Ord> {
    Leaf(Id),
    Branch {
        left: Box<Node<Id>>,
        right: Box<Node<Id>>,
    },
}
