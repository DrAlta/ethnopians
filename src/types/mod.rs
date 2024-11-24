#[cfg(not(feature = "macroquad"))]
mod vec2;
#[cfg(not(feature = "macroquad"))]
pub use vec2::{Vec2, vec2};


mod action_id;
pub use action_id::ActionID;
mod actor_id;
pub use actor_id::ActorID;
mod desire;
pub use desire::Desire;
mod steering;
pub use steering::{Steering, radians_to_u8, u8_to_radians};
mod time_index;
pub use time_index::TimeIndex;
