#![allow(dead_code)]
use crate::{IOTA, Number};

/// Standard buildup simulation.
/// Calculates if the entity crosses the 'tipping point' into an orgasm state.
pub fn bump(
    moralethic_appeal: Number,
    moralethic_adversion: Number,
    moralethic_importance: Number,

    satiety_threshold: Number,

    mut tension: Number,
    pleasure: Number,
    last_pleasure: Number,
){
    // Calculate the 'Taboo Weight': Both appeal and aversion (abs) increase mental load/arousal.
    let moralethic_tension = compute_moralethic_tension(moralethic_appeal, moralethic_adversion, moralethic_importance);

    // The threshold for release is the sum of moral friction and physical/psychological fullness (satiety).
    let tipping_point =  moralethic_tension + satiety_threshold;

    // Change in pleasure since the last tick.
    let pleasure_spike = pleasure - last_pleasure;

    // Trigger check: If current tension plus the new spike exceeds the limit, trigger immediate release.
    if  (tension + pleasure_spike) > tipping_point {
        let moralethic_intesity = (moralethic_appeal + moralethic_adversion.abs()) * moralethic_importance;
        let orgasm_intensity = tension + moralethic_intesity;
        orgasm(orgasm_intensity);
    } else {
        // Logistic dampening: Tension growth slows down as it approaches the tipping point.
        if tipping_point != Number::ZERO {
            tension *= (tipping_point - tension) / tipping_point;
        };
        // The 'Ache': Losing pleasure (abs spike) increases tension just as much as gaining it.
        tension += pleasure_spike.abs() * (Number::ONE + moralethic_tension);
    }
    // Hedonic Adaptation: Base pleasure level provides a diminishing 'creep' to tension.
    let pleasure_creep = pleasure.sqrt();

    tension += pleasure_creep
}

/// P-Type (Pleasure-driven): The orgasm is a resonant state fueled by active pleasure input.
fn pleasure_type_orgasm(
    moralethic_appeal: Number,
    moralethic_adversion: Number,
    moralethic_importance: Number,

    satiety_threshold: Number,

    mut tension: Number,
    pleasure: Number,
    last_pleasure: Number,

    mut orgasn_intensity: Number,
    mut over_taxation: Number,
){
    // Sensation 'Gluttony': Any pleasure received during orgasm builds the refractory debt.
    over_taxation += pleasure;

    let moralethic_tension = compute_moralethic_tension(moralethic_appeal, moralethic_adversion, moralethic_importance);
    let tipping_point =  moralethic_tension + satiety_threshold;

    // The 'Leverage': How far we are above the limit.
    let tension_reach = tension - tipping_point;
    let pleasure_mult = tension_reach.max(Number::ZERO);

    // Resonance: Pleasure is multiplied by how far 'over the edge' the entity is.
    orgasn_intensity += pleasure * pleasure_mult;
    
    // Drag: If tension falls below the tipping point, intensity is penalized (subtraction).
    orgasn_intensity +=  tension_reach.min(Number::ZERO);

    // Standard tension management during the state.
    if tipping_point != Number::ZERO {
        tension *= (tipping_point - tension) / tipping_point;
    };

    let pleasure_spike = pleasure - last_pleasure;

    tension += pleasure_spike.abs() * (Number::ONE + moralethic_tension);
}

/// T-Type (Tension-driven): The orgasm is a discharge event fueled by structural pressure.
fn tension_type_orgasm(
    moralethic_appeal: Number,
    moralethic_adversion: Number,
    moralethic_importance: Number,

    satiety_threshold: Number,

    mut tension: Number,
    pleasure: Number,
    last_pleasure: Number,

    mut orgasn_intensity: Number,
    mut over_taxation: Number,

    mut tension_high_water_mark: Number,

){
    // Accumulate refractory debt from the input pleasure.
    over_taxation += pleasure;

    let moralethic_tension = compute_moralethic_tension(moralethic_appeal, moralethic_adversion, moralethic_importance);

    let tipping_point =  moralethic_tension + satiety_threshold;

    let tension_reach = tension - tipping_point;



    // Tracking the peak: Intensity depends on pushing higher than the previous mark.
    tension_high_water_mark = tension_high_water_mark.max(tension_reach);

    let pleasure_spike = pleasure - last_pleasure;
    tension += pleasure_spike.abs() * (Number::ONE + moralethic_tension);

    // Momentum Logic: If current tension is rising vs the peak mark, intensity grows. 
    // If tension is falling, intensity drops (Tension Spike becomes negative).
    let tension_spike = tension - tension_high_water_mark;
    orgasn_intensity +=  tension_spike;

    if tipping_point != Number::ZERO {
        tension *= (tipping_point - tension) / tipping_point;
    };
}


/// State Check: Determines if the orgasm phase has naturally concluded.
fn orgasm_ended_ka(
    moralethic_appeal: Number,
    moralethic_adversion: Number,
    moralethic_importance: Number,

    satiety_threshold: Number,

    tension: Number,

    orgasn_intensity: Number,
) -> bool {
    let moralethic_tension = compute_moralethic_tension(moralethic_appeal, moralethic_adversion, moralethic_importance);

    let tipping_point =  moralethic_tension + satiety_threshold;

    // Ends when tension has fallen back and intensity has drained below zero.
    tension < tipping_point && orgasn_intensity < Number::ZERO

}

/// Refractory Period: Drains the 'Over-Taxation' debt over time.
fn refactory(
    mut over_taxation: Number,
    refactory_decay_rate: Number,
){
    // Exponential decay of sensitivity debt.
    over_taxation *= refactory_decay_rate;
    // Once debt is negligible (IOTA), the system resets.
    if over_taxation < *IOTA {
        end_refactory_state()
    }
}

fn orgasm(_orgasm_intensity: Number){}

fn end_refactory_state(){}

fn compute_moralethic_tension(
    moralethic_appeal: Number,
    moralethic_adversion: Number,
    moralethic_importance: Number,
)-> Number {
    let base = (moralethic_appeal - moralethic_adversion)
        .max(moralethic_appeal.max(Number::ZERO).sqrt())
        .max(moralethic_adversion.max(Number::ZERO).sqrt());

    base * moralethic_importance.abs()
    // + moralethic_appeal.abs() + moralethic_adversion.abs();

}
