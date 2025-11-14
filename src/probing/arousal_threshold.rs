use crate::{
    probing::{apprehension_delta, arousal_delta, MassageSkill, Massagee, Masseuse},
    Number, IOTA,
};

/// Calculate the minimum threshold for the massagee's arousal level, based on their current state, the masseuse's skills, and the massage skills being used.
/// This function is used to determine when the massagee's arousal level drops below the limit that she will allow the massage to continue.
/// What happens after depends on her personality, does to demand an erotic encouter, does she seduve him back, does she chicken out and leave or something else?
pub fn arousal_threshold(
    tame_skill: &MassageSkill,
    erotic_skill: &MassageSkill,
    peek_arousal: Number,
    masseuse: &Masseuse,
    massagee: &Massagee,
) -> Number {
    // Calculate the change in apprehension due to a tame massage.
    // This is used to determine how many tame massages are needed to offset the apprehension gained from an erotic massage.
    let tame_apprehension_loss = apprehension_delta(masseuse, massagee, tame_skill);
    // Calculate the change in apprehension due to an erotic massage.
    let erotic_apprehension_gain = apprehension_delta(masseuse, massagee, erotic_skill);

    // Calculate the number of tame massages needed to offset the apprehension gained from an erotic massage.
    // If the tame apprehension loss is too small, use a default value of 2 to avoid division by zero.
    let num_tame_massages = if tame_apprehension_loss <= *IOTA {
        (erotic_apprehension_gain / tame_apprehension_loss).ceil() + Number::ONE
    } else {
        Number::TWO
    };

    // Calculate the change in arousal due to a tame massage.
    let tame_arousal_loss = arousal_delta(masseuse, massagee, tame_skill);
    // Calculate the total arousal lost due to the tame massages needed to offset the apprehension gained from an erotic massage.
    let arousal_lost = num_tame_massages * tame_arousal_loss;

    // Calculate the factor that adjusts the arousal threshold based on the massagee's relationship with the masseuse.
    // This is based on the idea that the massagee's relationship with the masseuse affects their sensitivity to arousal drops.
    let linear_factor = (&massagee.arousal_threshold_factor_slope
        * &massagee.relationship_with_masseuse)
        + &massagee.arousal_threshold_factor_base;

    // Calculate the threshold for the massagee's arousal level, based on their peak arousal level and the arousal lost due to the tame massages.
    // The linear factor is used to adjust the threshold based on the massagee's relationship with the masseuse.
    peek_arousal - (arousal_lost * linear_factor)
}
