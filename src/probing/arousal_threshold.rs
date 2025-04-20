use crate::{probing::{apprehension_delta, arousal_delta, MassageSkill, Massagee, Masseuse}, Number, IOTA};


pub fn arousal_threshold(
    tame_skill: &MassageSkill,
    erotic_skill: &MassageSkill,
    peek_arousal: Number,
    masseuse: &Masseuse,
    massagee: &Massagee,
) -> Number {
    let tame_apprehension_loss = apprehension_delta(masseuse, massagee, tame_skill);
    let erotic_apprehension_gain = apprehension_delta(masseuse, massagee, erotic_skill);
    let num_tame_massages = if tame_apprehension_loss <= IOTA {
        (erotic_apprehension_gain / tame_apprehension_loss).ceil() + Number::ONE
    } else {
        Number::TWO
    };

    let tame_arousal_loss = arousal_delta(masseuse, massagee, tame_skill);
    let arousal_lost = num_tame_massages * tame_arousal_loss;

    let linear_factor = (&massagee.arousal_threshold_factor_slope * &massagee.relationship_with_masseuse) + &massagee.arousal_threshold_factor_base;

    peek_arousal - (arousal_lost * linear_factor)

}