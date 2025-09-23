use crate::Number;

// Define a struct to represent the massagee's state, including their current apprehension, arousal, and relationship with the masseuse.
// The threshold bases and slopes are used to calculate the thresholds for erotic encounters, apprehension, and arousal.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Massagee {
    // The massagee's current level of apprehension.
    pub apprehension: Number,
    // The massagee's current level of arousal.
    pub arousal: Number,
    // The base and slope values used to calculate the threshold for an erotic encounter.
    // A higher base value means the massagee is more open to erotic encounters, while a higher slope value means they become more open as their relationship with the masseuse deepens.
    pub erotic_encounter_threshold_base: Number,
    pub erotic_encounter_threshold_slope: Number,
    // The base and slope values used to calculate the threshold for apprehension.
    // A higher base value means the massagee is more tolerant of apprehension in new relationships.
    // While the higher the slope value means their tolerance of apprehensiion grow faster as their relationship with the masseuse deepens.
    pub apperhension_threshold_base: Number,
    pub apperhension_threshold_slope: Number,
    // The base and slope values used to calculate the factor that adjusts the minimum arousal threshold.
    // A higher base value means the massagee is more tolerant of her arousal dropping in new relationships.
    // While the higher the slope value means their tolerance of drops in her arousal grows faster as their relationship with the masseuse deepens.
    pub arousal_threshold_factor_base: Number,
    pub arousal_threshold_factor_slope: Number,
    // The massagee's current relationship level with the masseuse.
    pub relationship_with_masseuse: Number,
}
