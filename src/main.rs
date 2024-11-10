mod types;
pub use types::{ActionID, ActorID, Desire, TimeIndex};

pub mod behavior_tree;
mod brothel;
#[cfg(feature = "combat")]
mod combat;
pub mod social_sim;
mod trauma;

pub mod sqrt;

pub type Number = f64;

fn main() {
    println!("Hello, world!");
}
