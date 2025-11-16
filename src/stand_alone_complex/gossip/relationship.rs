use crate::Number;
/// Represents the direct relationship between this character and another character.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Relationship {
    /// The affection level towards the other character.
    /// Range: -1.0 (strong dislike) to 1.0 (strong liking).
    pub affection: Number,

    /// The trust level towards the other character.
    /// Range: 0.0 (no trust) to 1.0 (complete trust).
    pub trust: Number,
}
