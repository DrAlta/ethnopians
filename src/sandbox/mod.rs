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
pub mod ai;
//pub mod interaction;

mod acts;
use std::sync::Arc;

pub use acts::Acts;
//pub mod collision;
pub mod forth;
mod item;
pub use item::Item;
mod location;
pub use location::Location;
pub mod movement;
mod r#return;
pub use r#return::Return;

mod sandbox;
pub use sandbox::within_range;
pub mod world;

#[cfg(feature = "bevy")]
pub type EntityId = bevy::prelude::Entity;
#[cfg(not(feature = "bevy"))]
pub type EntityId = u64;

pub type ItemClass = Arc<String>;


#[cfg(feature = "bevy")]
use bevy::prelude::Component;
#[cfg(not(feature = "bevy"))]
use macros::Component;

#[cfg(feature = "bevy")]
use bevy::prelude::Event;
#[cfg(not(feature = "bevy"))]
use macros::Event;