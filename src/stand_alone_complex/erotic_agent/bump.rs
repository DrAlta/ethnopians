use crate::{Number, IOTA};

use crate::stand_alone_complex::orgasm::compute_moralethic_tension;

use super::EroticAgent;

pub enum BumpResponce {
    Orgasm { orgasm_intensity: Number },
    NoOrgasm,
}

impl EroticAgent {
    pub fn bump(
        &mut self,
        moralethic_appeal: Number,
        moralethic_adversion: Number,
        moralethic_importance: Number,
        pleasure: Number,
    ) -> BumpResponce {
        // Calculate the 'Taboo Weight': Both appeal and aversion (abs) increase mental load/arousal.
        let moralethic_tension = compute_moralethic_tension(
            moralethic_appeal,
            moralethic_adversion,
            moralethic_importance,
        );

        // The threshold for release is the sum of moral friction and physical/psychological fullness (satiety).
        let tipping_point = moralethic_tension + self.satiety_threshold;

        // Change in pleasure since the last tick.
        let pleasure_spike = pleasure - self.last_pleasure;

        let ret =

    // Trigger check: If current tension plus the new spike exceeds the limit, trigger immediate release.
    if (self.tension + pleasure_spike) > tipping_point {
        let moralethic_intesity =
            (moralethic_appeal + moralethic_adversion.abs()) * moralethic_importance;
        let orgasm_intensity = self.tension + moralethic_intesity;
        BumpResponce::Orgasm { orgasm_intensity }
    } else {
        // Logistic dampening: Tension growth slows down as it approaches the tipping point.
        if tipping_point != Number::ZERO {
            self.tension *= (tipping_point - self.tension) / tipping_point;
        };
        // The 'Ache': Losing pleasure (abs spike) increases tension just as much as gaining it.
        self.tension += pleasure_spike.abs() * (Number::ONE + moralethic_tension);
        BumpResponce::NoOrgasm
    };
        // Hedonic Adaptation: Base pleasure level provides a diminishing 'creep' to tension.
        let pleasure_creep = pleasure.sqrt();

        self.tension += pleasure_creep;
        self.last_pleasure = pleasure;
        ret
    }
}
