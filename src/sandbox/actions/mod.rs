/*mod action_id;
pub use action_id::ActionId;
*/
mod action_result;
pub use action_result::{ActionResult, Result};
mod goto;
pub use goto::{goto_system, GotoRequest};
mod posible_actions_request;
pub use posible_actions_request::PosibleActionsRequest;
mod posible_actions_responce;
pub use posible_actions_responce::PosibleActionsResponce;
mod use_object;
pub use use_object::{use_object_system, UseRequest};
mod use_on;
pub use use_on::{use_on_system, UseOnRequest};
