//mod execute_commands;
//mod step_sim;
//mod world;
//pub use world::World;

use crate::sandbox::{Component, Item};
use crate::{Number, Vec2};

#[derive(Component, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Energy(pub i32);

#[derive(Component, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Hp(pub i32);

#[derive(Component, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Size {
    pub width: i32,
    pub height: i32,
}

#[derive(Component, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Type(pub Item);

#[derive(Component, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Movement {
    pub target: Vec2,
    pub speed: Number,
}
