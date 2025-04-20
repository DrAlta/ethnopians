use crate::{Number, probing::{MassageSkill, Massagee, Masseuse}};

/// Calculate the change in the massagee's arousal based on their current state, the masseuse's skills, and the massage skill being used.
/// This function is used to determine how the massagee's arousal changes in response to different massage techniques.
pub fn arousal_delta(
    masseuse: &Masseuse,
    massagee: &Massagee,
    skill: & MassageSkill
) -> Number {
    skill.arousal.calculate_delta(masseuse, massagee)
}
