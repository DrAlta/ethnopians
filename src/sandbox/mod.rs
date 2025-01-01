pub const MAX_ENERGY: i16 = 100;

pub mod interaction;

mod acts;
pub use acts::Acts;
pub mod bt;
pub mod collision;
mod command;
pub use command::Command;
pub mod forth;
mod item;
pub use item::Item;
mod location;
pub use location::Location;
mod movement;
pub use movement::{process_movement, Prev};
mod r#return;
pub use r#return::Return;
mod sandbox;
use sandbox::within_range;
mod use_object;
pub use use_object::UseObject;
mod world;
pub use world::World;

pub type ObjectId = usize;
