use std::collections::BTreeMap;

use qol::AddOrInsert;

use crate::{trauma::{Event, TraumaExperience, TraumaExperienceId, TraumaExperienceType}, Number};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Psych {
    // Mapping of experience types to the number of times experienced
    num_experiences: BTreeMap<TraumaExperienceType, u32>,
    // Mapping of experience types to the number of good experiences
    num_good_experiences: BTreeMap<TraumaExperienceType, u32>,
    recent_experiences: Vec<TraumaExperience>,
    // Location in the comfort zone (average of how well handled experiences)
    location_in_comfort_zone: Number,
    // modifer to the event's base_traumatic_stress to get the actual traumatic_stress she experinces
    //  the value in the map is (gradient, y_intercept)
    // actual_stress = gradient * base_stess + y_intercept
    ability_to_handle_emotional_experience_type:
        BTreeMap<TraumaExperienceType, (Number, Number)>,
}

impl Psych {
    pub fn get_recent_experiences(&self) -> &Vec<TraumaExperience> {
        &self.recent_experiences
    }
    pub fn get_num_good_experiences(&self, experience_type: &TraumaExperienceType) -> u32 {
        let Some(x) = self.num_good_experiences.get(experience_type) else {
            return 0;
        };
        x.clone()
    }
    pub fn new() -> Psych {
        Psych {
            num_experiences: BTreeMap::new(),
            num_good_experiences: BTreeMap::new(),
            recent_experiences: Vec::new(),

            location_in_comfort_zone: 0.into(),
            ability_to_handle_emotional_experience_type: BTreeMap::new(),
        }
    }
    fn set_ability_to_handle_emotional_experience_type(
        &mut self,
        experience_type: TraumaExperienceType,
        value: (Number, Number),
    ) -> Option<(Number, Number)> {
        self.ability_to_handle_emotional_experience_type
            .insert(experience_type, value)
    }
    fn get_ability_to_handle_emotional_experience_type(
        &self,
        experience_type: &TraumaExperienceType,
    ) -> (Number, Number) {
        let Some(x) = self
            .ability_to_handle_emotional_experience_type
            .get(experience_type)
        else {
            return (1.into(), 0.into());
        };
        (x.0.clone(), x.1.clone())
    }

    /// Calculate how the inital trauma of the event using the line formula
    pub fn calc_traumatic_stress(&self, experience_type:&TraumaExperienceType , base_traumatic_stressfullness: Number) -> Number {
        let (gradient, y_intercept) =
            self.get_ability_to_handle_emotional_experience_type(experience_type);
        let traumatic_stress = gradient * base_traumatic_stressfullness + y_intercept;
        traumatic_stress
    }
    pub fn insert_trauma_experience(&self, _trauma_experince_id: TraumaExperienceId, _trauma_experince: TraumaExperience) -> Option<TraumaExperience> {
        todo!()
    } 
    pub fn get_trauma_experience(&self, _trauma_experince_id: &TraumaExperienceId) -> Option<&TraumaExperience> {
        todo!()
    } 
    pub fn get_trauma_experience_mut(&self, _trauma_experince_id: &TraumaExperienceId) -> Option<&mut TraumaExperience> {
        todo!()
    } 

    pub fn handle_trauma_event(&mut self, trauma_experince_id: TraumaExperienceId, event: Event) {
        match event {
            Event::Support { stressfulness, suppertiveness } => todo!(),
            Event::Trauma { trauma_type, base_traumatic_stressfulness } => {
                // Update the number of experiences of this type
                self.num_experiences
                    .add_or_insert(trauma_type.clone(), 1);
                let traumatic_stress = self.calc_traumatic_stress(&trauma_type, base_traumatic_stressfulness);

                let coping_support = match self.get_trauma_experience(&trauma_experince_id) {
                    Some(experience) => experience.get_current_coping_support(),
                    None => Number::ZERO,
                };

                // how much trauma was caused by the event
                let trauma = traumatic_stress - coping_support;

                // the effects of that trauma

                // If it was out of her confortzone it was a bad experince
                // use 1 / confortzone to map 0 to inf to inf to 0
                let trauma_threshold = Number::ONE / self.location_in_comfort_zone.clone();
                let bad_experience_ka = trauma > trauma_threshold;

                // effects of her ability to handle that type of experince
                if bad_experience_ka {
                    // decrease abiity to handle. the father out of her comfortzone the bigger the decrease use every time
                    let x = traumatic_stress.clone().max(trauma.clone());
                    let trauma_overload = x.clone() - trauma_threshold.clone();

                    let gradient_delta = Number::try_from(
                        (1.0 - ((Into::<f64>::into(&trauma_overload) + 1.0).powi(-1))) * 2.0,
                    )
                    .unwrap();

                    let (gradient, y_intercept) =
                        self.get_ability_to_handle_emotional_experience_type(&trauma_type);
                    let new_gradient = gradient + gradient_delta;
                    let new_y_intercept = y_intercept * (x / trauma_threshold);

                    self.set_ability_to_handle_emotional_experience_type(
                        trauma_type.clone(),
                        (new_gradient, new_y_intercept),
                    );
                } else {
                    // Calculate the increase in her ability to handle this type of experience
                    // the farther away fom her confort zone the less of an improvment in her ability
                    // comfort_zone_factor is 1 is she is at the center of her comfortzone and decreases the father out she is
                    let comfort_zone_factor = Number::ONE
                        / (Number::ONE
                            + (Number::HALF * self.location_in_comfort_zone.clone()));
                    let (gradient, y_intercept) =
                        self.get_ability_to_handle_emotional_experience_type(&trauma_type);
                    // the 4 is fairly arbitray. the biger the value the slowing value grows
                    let base_gradient_improvement_percent = traumatic_stress.clone()
                        / (traumatic_stress.clone() + Number::FOUR);
                    let new_gradient = gradient.clone()
                        - (gradient * base_gradient_improvement_percent * comfort_zone_factor);

                    let good_experience_factor = Number::try_from(
                        ((self.get_num_good_experiences(&trauma_type) + 1) as f64).sqrt(),
                    )
                    .unwrap();
                    let perscent_over_threshold =
                        (traumatic_stress.clone() / trauma_threshold) - Number::ONE;
                    let new_y_intercept = y_intercept.clone()
                        - (y_intercept * perscent_over_threshold * good_experience_factor);
                    self.set_ability_to_handle_emotional_experience_type(
                        trauma_type.clone(),
                        (new_gradient, new_y_intercept),
                    );
                }
                // add the event to the experince
                match self.get_trauma_experience_mut(&trauma_experince_id) {
                    Some(experience) => experience.add_event(trauma, base_traumatic_stressfulness),
                    None => {
                        let experience = TraumaExperience::simple_new(trauma, base_traumatic_stressfulness, trauma_type);
                        self.insert_trauma_experience(trauma_experince_id, experience);
                    },
                }
            },
        }
    }
}
