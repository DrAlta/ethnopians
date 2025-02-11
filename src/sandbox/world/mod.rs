//mod execute_commands;
//mod step_sim;
//mod world;
//pub use world::World;

use bevy::prelude::*;

pub use crate::sandbox::Item;

#[derive(Component, Debug)]
pub struct Energy(pub i32);

#[derive(Component, Debug)]
pub struct Hp(pub i32);

#[derive(Component, Debug)]
pub struct Size {
    pub width: i32,
    pub height: i32,
}

#[derive(Component, Debug)]
pub struct Type(pub Item);

#[derive(Component, Debug)]
pub struct Movement {
    pub target: Vec2,
    pub speed: f32,
}
