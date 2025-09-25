use crate::sandbox::ai::BlackboardKey;

#[derive(Debug, Clone, PartialEq)]
pub enum Node {
    // theseare memory selector and memory sequence 
    Selector{children: Vec<Self>},
    Sequence{children: Vec<Self>},
    // resets tircks its `consition` branch and 
    //if returns Success then continues  executing the `task` branch. that is tick with the revious that it gave
    // if `condition` failed the it restarts exucuting the `task` branch
    // that is ticks it with None for it's state
//    Reset{condition: Box<Self>, task:Box<Self>},
    Parallel{children: Vec<Self>, needed_successed: usize, failure_abort_limit: usize},
    // #####
    // # Decorator
    // ###
    Inverter{child: Box<Self>},
    // #####
    // # Action
    // ###
    // takes two Blackboard keys that points to ItemClass
    Combine{key_to_direct_item_class: BlackboardKey, key_to_indirect_item_class: BlackboardKey},

    // #####
    // # Condition
    // ###
    // takes a Blackboard key that points to an ItemClass and u8 of the number to compare to
    InventoryGE{key_to_item_class: BlackboardKey, amount: i32},
}
impl Node {
    pub fn name(&self) -> &'static str {
        match self {
            Node::Selector { .. } => "Selector",
            Node::Sequence { .. } => "Sequence",
            Node::Parallel { .. } => "Parallel",
            Node::Inverter { .. } => "Inverter",
            Node::Combine { .. } => "Combine",
            Node::InventoryGE { .. } => "InventoryGE",
        }
    }
}
