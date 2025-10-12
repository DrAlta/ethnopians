/*
parent.Tick(none)-> call(parentstate1, child, None)
child.tick(none) -> Wait(childstate)
parent.uptick(parentstate1, Wait(childstate)) ->wait(parentstate2{childstate})
parent.Tick(parentstate2{childstate}) -> call(parentstate3, child, childstate)
child.tick(childstate) -> Success
parent.Uptick(parentstate3, Success)
*/

mod prayer;
pub use prayer::Prayer;
mod state;
pub use state::State;
mod status;
pub use status::Status;
mod node;
pub use node::Node;
