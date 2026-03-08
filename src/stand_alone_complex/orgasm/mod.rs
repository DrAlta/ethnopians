#![allow(dead_code)]
use crate::{IOTA, Number};

pub fn bump(
    moralethic_appeal: Number,
    moralethic_adversion: Number,
    moralethic_importance: Number,

    satiety_threshold: Number,

    mut tension: Number,
    pleasure: Number,
    last_pleasure: Number,
){
    let moralethic_tension = (((moralethic_appeal - moralethic_adversion) * moralethic_importance) + moralethic_appeal + moralethic_adversion.abs()).abs();

    let tipping_point =  moralethic_tension + satiety_threshold;


    let pleasure_spike = pleasure - last_pleasure;

    if  (tension + pleasure_spike) > tipping_point {
        let moralethic_intesity = (moralethic_appeal + moralethic_adversion.abs()) * moralethic_importance;
        let orgasm_intensity = tension + moralethic_intesity;
        orgasm(orgasm_intensity);
    } else {
        if tipping_point != Number::ZERO {
            tension *= (tipping_point - tension) / tipping_point;
        };
        tension += pleasure_spike.abs() * (Number::ONE + moralethic_tension);
    }
    let pleasure_creep = pleasure.sqrt();

    tension += pleasure_creep
}


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

    over_taxation += pleasure;

    let moralethic_tension = (((moralethic_appeal - moralethic_adversion) * moralethic_importance) + moralethic_appeal + moralethic_adversion.abs()).abs();

    let tipping_point =  moralethic_tension + satiety_threshold;

    let tension_reach = tension - tipping_point;
    let pleasure_mult = tension_reach.max(Number::ZERO);

    orgasn_intensity += pleasure * pleasure_mult;
    
    orgasn_intensity +=  tension_reach.min(Number::ZERO);

    if tipping_point != Number::ZERO {
        tension *= (tipping_point - tension) / tipping_point;
    };

    let pleasure_spike = pleasure - last_pleasure;

    tension += pleasure_spike.abs() * (Number::ONE + moralethic_tension);
}

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

    over_taxation += pleasure;

    let moralethic_tension = (((moralethic_appeal - moralethic_adversion) * moralethic_importance) + moralethic_appeal + moralethic_adversion.abs()).abs();

    let tipping_point =  moralethic_tension + satiety_threshold;

    let tension_reach = tension - tipping_point;



    tension_high_water_mark = tension_high_water_mark.max(tension_reach);

    
    let pleasure_spike = pleasure - last_pleasure;

    tension += pleasure_spike.abs() * (Number::ONE + moralethic_tension);

    let tension_spike = tension - tension_high_water_mark;

    orgasn_intensity +=  tension_spike;


    if tipping_point != Number::ZERO {
        tension *= (tipping_point - tension) / tipping_point;
    };

}

fn orgasm_ended_ka(
    moralethic_appeal: Number,
    moralethic_adversion: Number,
    moralethic_importance: Number,

    satiety_threshold: Number,

    mut tension: Number,
    pleasure: Number,
    last_pleasure: Number,

    mut orgasn_intensity: Number,
    mut over_taxation: Number,
) -> bool {
    let moralethic_tension = (((moralethic_appeal - moralethic_adversion) * moralethic_importance) + moralethic_appeal + moralethic_adversion.abs()).abs();

    let tipping_point =  moralethic_tension + satiety_threshold;

    tension < tipping_point && orgasn_intensity < Number::ZERO

}

fn refactory(
    mut over_taxation: Number,
    refactory_decay_rate: Number,
){
    over_taxation *= refactory_decay_rate;
    if over_taxation < *IOTA {
        end_refactory_state()
    }
}

fn orgasm(_orgasm_intensity: Number){}

fn end_refactory_state(){}