use std::collections::BTreeMap;

use qol::AddOrInsert;

use fraction64::Fraction;

use super::{Event, TraumaExperience, TraumaExperienceType};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone)]
pub struct Stage1EmotionalExperience {
    // Type of emotional experience
    experience_type: TraumaExperienceType,
    // how_traumaticly_stressful_the_event_is - how_well_she_can_handle_that_type
    traumatic_stress: Fraction,
 }

 #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone)]
 pub struct Psych {
    // Mapping of experience types to the number of times experienced
    num_experiences: BTreeMap<TraumaExperienceType, u32>,
    // Mapping of experience types to the number of good experiences
    num_good_experiences: BTreeMap<TraumaExperienceType, u32>,
    recent_experiences: Vec<TraumaExperience>,
    // Location in the comfort zone (average of how well handled experiences)
    location_in_comfort_zone: Fraction,
    // modifer to the event's base_traumatic_stress to get the actual traumatic_stress she experinces
    //  the value in the map is (gradient, y_intercept) 
    // actual_stress = gradient * base_stess + y_intercept
    ability_to_handle_emotional_experience_type: BTreeMap<TraumaExperienceType, (Fraction, Fraction)>,
}


impl Psych {
    pub fn get_recent_experiences(&self) -> &Vec<TraumaExperience> {
        &self.recent_experiences
    }
    pub fn get_num_good_experiences(&self, experience_type: &TraumaExperienceType ) -> u32 {
        let Some(x) = self.num_good_experiences.get(experience_type) else {
            return 0
        };
        x.clone()
    }
    pub fn new() -> Psych {
        Psych
        {
            num_experiences: BTreeMap::new(),
            num_good_experiences: BTreeMap::new(),
            recent_experiences: Vec::new(),

            location_in_comfort_zone: 0.into(),
            ability_to_handle_emotional_experience_type: BTreeMap::new(),
        }
    }
    fn set_ability_to_handle_emotional_experience_type(&mut self, experience_type: TraumaExperienceType, value: (Fraction, Fraction)) -> Option<(Fraction, Fraction)> {
        self.ability_to_handle_emotional_experience_type.insert(experience_type, value)
    }
    fn get_ability_to_handle_emotional_experience_type(&self, experience_type: &TraumaExperienceType) -> (Fraction, Fraction) {
        let Some(x) = self.ability_to_handle_emotional_experience_type.get(experience_type) else {
            return (1.into(), 0.into())
        };
        (x.0.clone(), x.1.clone())
        

    }

    pub fn handle_experience_stage_1(&mut self, event: Event) -> Stage1EmotionalExperience{
        let experience_type = event.get_trauma_type().clone();
        // Update the number of experiences of this type
        self.num_experiences.add_or_insert(experience_type.clone(), 1);
        

        // Calculate how the inital trauma of the event using the line formula
        let (gradient, y_intercept) = self.get_ability_to_handle_emotional_experience_type(&experience_type);
        let traumatic_stress = gradient * event.get_base_traumatic_stress() + y_intercept ;

        Stage1EmotionalExperience{experience_type, traumatic_stress}
    }
    pub fn handle_experience_stage_2 (
        &mut self, 
        Stage1EmotionalExperience{
            experience_type, 
            traumatic_stress
        }:Stage1EmotionalExperience, 
        coping_support: Fraction
    ) {
        // how much trauma was caused by the event
        let trauma = traumatic_stress.clone() - coping_support;

        // the effects of that trauma
         
        // If it was out of her confortzone it was a bad experince
        // use 1 / confortzone to map 0 to inf to inf to 0
        let trauma_threshold = Fraction::from(1) / self.location_in_comfort_zone.clone();
        let bad_experience_ka = trauma > trauma_threshold;

        // effects of her ability to handle that type of experince
        if bad_experience_ka {
            // decrease abiity to handle. thefather out of her comfortzone the bigger the decrease use every time 
            let x = traumatic_stress.clone().max(trauma.clone());
            let trauma_overload = x.clone() - trauma_threshold.clone();

            let gradient_delta = Fraction::try_from(
                (
                    1.0 - (
                        (
                            Into::<f64>::into(&trauma_overload) + 1.0
                        ).powi(-1)
                    )
                ) 
                * 2.0
            ).unwrap();

            let (gradient, y_intercept) = self.get_ability_to_handle_emotional_experience_type(&experience_type);
            let new_gradient = gradient + gradient_delta;
            let new_y_intercept = y_intercept * ( x / trauma_threshold );

            self.set_ability_to_handle_emotional_experience_type(experience_type.clone(), (new_gradient, new_y_intercept));

        } else {
            // Calculate the increase in her ability to handle this type of experience
            // the farther away fom her confort zone the less of an improvment in her ability
            // comfort_zone_factor is 1 is she is at the center of her comfortzone and decreases the father out she is
            let comfort_zone_factor =  Fraction::from(1) / ( Fraction::from(1) + (Fraction::try_new(1,2).unwrap() * self.location_in_comfort_zone.clone()));
            let (gradient, y_intercept) = self.get_ability_to_handle_emotional_experience_type(&experience_type);
            // the 4 is fearly arbitray. the biger the value the slowing value grows
            let base_gradient_improvement_percent = traumatic_stress.clone() / (traumatic_stress.clone() + Fraction::try_new(4,1).unwrap());
            let new_gradient = gradient.clone() - (gradient * base_gradient_improvement_percent * comfort_zone_factor);

            let good_experience_factor = Fraction::try_from(
                (
                    (
                        self.get_num_good_experiences(&experience_type) + 1
                    ) as f64
                )
                     .sqrt()).unwrap();
            let perscent_over_threshold = (traumatic_stress.clone() / trauma_threshold) - Fraction::from(1);
            let new_y_intercept = y_intercept.clone() - (y_intercept * perscent_over_threshold * good_experience_factor);
            self.set_ability_to_handle_emotional_experience_type(experience_type.clone(), (new_gradient, new_y_intercept));
        }
        self.recent_experiences.push(TraumaExperience::simple_new(experience_type, trauma, traumatic_stress))
    }
}

