mod types;
pub use types::{ActionID, ActorID, Desire, TimeIndex};
mod trauma;
pub mod sqrt;
#[cfg(feature = "combat")]
mod combat;

pub mod social_sim;

pub type Number = f64;

fn main() {
    println!("Hello, world!");
}
