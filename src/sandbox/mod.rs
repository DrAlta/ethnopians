pub const MAX_ENERGY: i16 = 100;

pub mod interaction;

mod acts;
pub use acts::Acts;
mod command;
pub use command::Command;
mod item;
pub use item::Item;
mod location;
pub use location::Location;
mod r#return;
pub use r#return::Return;
mod sandbox;
use sandbox::within_range;
mod use_object;
pub use use_object::UseObject;
mod world;
pub use world::World;
