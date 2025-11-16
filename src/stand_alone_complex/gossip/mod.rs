//! this was coded by Chad.  I''m reveiw it when I migrate it bevy
mod character;
pub use character::Character;
mod gossip_content;
pub use gossip_content::GossipContent;
mod gossip_impact;
pub use gossip_impact::GossipImpact;
mod opinion;
pub use opinion::Opinion;
mod opinion_change;
pub use opinion_change::OpinionChange;
mod personality_traits;
pub use personality_traits::PersonalityTraits;
mod relationship;
pub use relationship::Relationship;


/// A unique identifier type for each character in the game.
type CharId = usize;
