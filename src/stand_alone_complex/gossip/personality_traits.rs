use crate::Number;
/// Represents the personality traits of a character that influence their behavior and reactions
/// in the gossip system.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PersonalityTraits {
    /// How forgiving the character is when others share commonly held beliefs, even if untrue.
    /// Range: 0.0 (not forgiving) to 1.0 (very forgiving).
    pub forgiveness_for_common_beliefs: Number,

    /// The extent to which the character prefers information that confirms their own beliefs.
    /// Range: 0.0 (objective/open-minded) to 1.0 (strongly biased towards confirmation).
    pub confirmation_bias: Number,

    /// How easily the character's trust increases when they hear information that confirms
    /// their existing beliefs.
    /// Range: 0.0 (skeptical) to 1.0 (very gullible).
    pub gullibility_for_confirmation: Number,

    /// The tendency of the character to align with the opinions of others (conformity).
    /// Range: 0.0 (non-conformist) to 1.0 (highly conformist).
    pub conformity: Number,

    /// The character's natural skepticism towards new or contradicting information.
    /// Range: 0.0 (not skeptical/gullible) to 1.0 (highly skeptical).
    pub skepticism: Number,

    /// Determines whether the character weighs affection or trust more when considering others'
    /// opinions. Range: -1.0 (prioritize trust) to 1.0 (prioritize affection).
    pub opinion_weight_bias: Number,
}
