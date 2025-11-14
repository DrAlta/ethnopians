use crate::broadphase::bvh::Node;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum NodeType<Id>
{
    Leaf(Id),
    Branch {
        left: Box<Node<Id>>,
        right: Box<Node<Id>>,
    },
}
