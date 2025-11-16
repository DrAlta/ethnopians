use crate::Number;
use super::{Massagee, Masseuse};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Skillette {
    pub arousal_base: Number,
    pub arousal_slope: Number,

    pub apprehension_base: Number,
    pub apprehension_slope: Number,

    pub knowledge_base: Number,
    pub knowledge_slope: Number,

    pub skill_base: Number,
    pub skill_slope: Number,
}

impl Skillette {
    pub fn calculate_delta(&self, masseuse: &Masseuse, massagee: &Massagee) -> Number {
        // Calculate the component of change due to the massagee's current arousal level.
        let arousal_component = (&massagee.arousal * &self.arousal_slope) + &self.arousal_base;

        // Calculate the component of the change due to the massagee's current apprehension level.
        let apprehension_component =
            (&massagee.apprehension * &self.apprehension_slope) + &self.apprehension_base;

        // Calculate the bonus to the change due to the masseuse's knowledge of the massagee.
        let knowledge_bonus =
            (masseuse.knowledge_of_massagee * &self.knowledge_slope) + &self.knowledge_base;

        // Calculate the bonus to the change due to the masseuse's skill level.
        // This is based on the idea that a more skilled masseuse can keep arousal from deceasing more effectively.
        let skill_bonus = (masseuse.massage_skill * &self.skill_slope) + &self.skill_base;

        // Combine the components and bonuses to calculate the total change.
        arousal_component + apprehension_component + knowledge_bonus + skill_bonus
    }
}
