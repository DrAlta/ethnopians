use crate::{
    probing::{MassageSkill, Massagee, Masseuse},
    Number,
};

/// Calculate the change in the massagee's apprehension based on their current state, the masseuse's skills, and the massage skill being used.
/// This function is used to determine how the massagee's apprehension changes in response to different massage techniques.
pub fn apprehension_delta(
    masseuse: &Masseuse,
    massagee: &Massagee,
    skill: &MassageSkill,
) -> Number {
    skill.apprehension.calculate_delta(masseuse, massagee)
}
