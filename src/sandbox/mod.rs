//! use kelly criterion on the current const to purch to curent selling price
//! I want to buy N more then I want to filter out the ones that I think I'll be able to buy cheaper in th future
//! ```
//! type Number = f64;
//! fn predict_selling_price(time:Number) -> Number{todo!()};
//! fn predict_cost(time:Number) -> Number{todo!()};
//!
//! fn foo(number_want_to_buy: u16, mean_time_between_sells: Number, start_time:Number) -> u16 {
//!     let current_ratio = predict_selling_price(start_time) / predict_cost(start_time);
//!     let mut number_to_actualy_buy = 1;
//!     let mut time = start_time + mean_time_between_sells;
//!     for _ in 1..number_want_to_buy {
//!         let ratio = predict_selling_price(time) / predict_cost(time);
//!         if ratio < current_ratio {
//!             number_to_actualy_buy += 1;
//!         }
//!         time += mean_time_between_sells;
//!     }
//!     number_to_actualy_buy
//!     
//! }
//!
//!
//!

pub const MAX_ENERGY: i32 = 100;
//pub mod ai;
//pub mod interaction;

mod actions;
pub use actions::{use_object_system, ActionId};
mod acts;
pub use acts::Acts;
//pub mod collision;
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
/*
mod use_object;
pub use use_object::UseObject;
*/
mod world;

pub type EntityId = bevy::prelude::Entity;
pub type ItemClass = String;
