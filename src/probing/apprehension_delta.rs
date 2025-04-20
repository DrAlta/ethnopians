use crate::{Number, probing::{MassageSkill, Massagee, Masseuse}};


/// Calculate the change in the massagee's apprehension based on their current state, the masseuse's skills, and the massage skill being used.
/// This function is used to determine how the massagee's apprehension changes in response to different massage techniques.
pub fn apprehension_delta(
    masseuse: &Masseuse,
    massagee: &Massagee,
    skill: &MassageSkill,
) -> Number {
    // Calculate the component of the apprehension change due to the massagee's current arousal level.
    // This is based on the idea that the massagee's arousal level can affect their apprehension, and that different massage skills can have different effects on arousal.
    let arousal_component = (&massagee.arousal * &skill.apprehension_arousal_slope) + &skill.apprehension_arousal_base;

    // Calculate the component of the apprehension change due to the massagee's current apprehension level.
    // This is based on the idea that the massagee's current apprehension level can affect their future apprehension, and that different massage skills can have different effects on apprehension.
    let apprehension_component = (&massagee.apprehension * &skill.apprehension_apprehension_slope) + &skill.apprehension_apprehension_base;

    // Calculate the bonus to the apprehension change due to the masseuse's knowledge of the massagee.
    // This is based on the idea that a masseuse with more knowledge of the massagee can better tailor their technique to the massagee's needs, reducing apprehension.
    let knowledge_bonus = (masseuse.knowledge_of_massagee * &skill.apprehension_knowledge_slope) + &skill.apprehension_knowledge_base;

    // Calculate the bonus to the apprehension change due to the masseuse's skill level.
    // This is for symitry withe arousal_delta where  itbased on the idea that a more skilled masseuse can keep arousal from deceasing more effectively.
    let skill_bonus = (masseuse.massage_skill * &skill.apprehension_skill_slope) + &skill.apprehension_skill_base;

    // Combine the components and bonuses to calculate the total change in apprehension.
    arousal_component + apprehension_component + knowledge_bonus + skill_bonus
}