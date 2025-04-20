use crate::{Number, probing::{MassageSkill, Massagee, Masseuse}};

/// Calculate the change in the massagee's arousal based on their current state, the masseuse's skills, and the massage skill being used.
/// This function is used to determine how the massagee's arousal changes in response to different massage techniques.
pub fn arousal_delta(
    masseuse: &Masseuse,
    massagee: &Massagee,
    skill: & MassageSkill
) -> Number {
    // Calculate the component of the arousal change due to the massagee's current arousal level.
    // This is based on the idea that the massagee's current arousal level can affect their future arousal.
    let arousal_component = (&massagee.arousal * &skill.arousal_arousal_slope) + &skill.arousal_arousal_base;

    // Calculate the component of the arousal change due to the massagee's current apprehension level.
    // This is based on the idea that the massagee's current apprehension level can affect their arousal.
    let apprehension_component = (&massagee.apprehension * &skill.arousal_apprehension_slope) + &skill.arousal_apprehension_base;

    // Calculate the bonus to the arousal change due to the masseuse's knowledge of the massagee.
    // This is based on the idea that a masseuse with more knowledge of the massagee can better tailor their technique to the massagee's needs, increasing arousal.
    let knowledge_bonus = (masseuse.knowledge_of_massagee * &skill.arousal_knowledge_slope) + &skill.arousal_knowledge_base;

    // Calculate the bonus to the arousal change due to the masseuse's skill level.
    // This is based on the idea that a more skilled masseuse can keep arousal from deceasing more effectively.
    let skill_bonus = (masseuse.massage_skill * &skill.arousal_skill_slope) + &skill.arousal_skill_base;

        // Combine the components and bonuses to calculate the total change in arousal.
        arousal_component + apprehension_component + knowledge_bonus + skill_bonus
}
