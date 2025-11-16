use crate::Number;

/// Represents this character's perception of how one character feels about another character.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Opinion {
    /// The believed affection that the subject character has towards the target character.
    /// Range: -1.0 (believes the subject strongly dislikes the target) to 1.0 (believes the
    /// subject strongly likes the target).
    pub affection: Number,
}
