use super::{Item, Location};

mod use_object;

#[derive(Debug,PartialEq, PartialOrd, Clone)]
pub enum Command{
    AddItem{item: Item, loc: Location},
    Remove(usize),
    Damage{agent: usize, ammout: i16},
    Rest{agent_idx: usize, ammount: i16},
    Heal{agent_idx: usize, ammount: i16},
}
