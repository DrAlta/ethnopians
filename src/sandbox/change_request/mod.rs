//! 1) all the actions procress the actions requests and for the successful action send out a ChangeRequest(ActionHash, BTreeSet<ContentousEntitieIds>, change )
//! 2) a system goes thru the ChangeRequest events and looks for any that contitous over the same
//!
//! sort_events_into_ let change_requests_by_contentous_entities = HashMap<ContentousEntitieIds, Vec<ChangeRequestId>>
//! for sort the changeRequests by their hashs
//! go down the sorted by hash list and if the request if righting remove it and update the
//!     let mut cleared = true;
//!    for x in request.contentous {
//!         if let Some(thing) = change_requests_by_contentous_entities.get_mut(x) {
//!             thing.remove(request)
//!             if !thing.is_Empty() {
//!                 cleared = false;
//!             }
//!         }
//!    }
//!     if cleared {
//!         send_event_for_Change_to_happen
//!     }
//!
mod change_request;
pub use change_request::ChangeRequest;
mod change_request_system;
pub use change_request_system::change_request_system;
mod change_conflict;
pub mod change_systems;
pub use change_conflict::ChangeConflict;
mod changes;
pub use changes::*;
mod dispatch;
pub use dispatch::Dispatch;
//mod events;
//pub use events::*;
