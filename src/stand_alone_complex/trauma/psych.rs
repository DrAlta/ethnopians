use std::collections::BTreeMap;

use qol::AddOrInsert;

use crate::Number;

use super::{Event, TraumaExperience, TraumaExperienceId, TraumaExperienceType};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Psych {
    // Mapping of experience types to the number of times experienced
    num_experiences: BTreeMap<TraumaExperienceType, u32>,
    // Mapping of experience types to the  number of traumatic experince of that they where are to successfully manage
    num_good_experiences: BTreeMap<TraumaExperienceType, u32>,
    recent_experiences: BTreeMap<TraumaExperienceId, TraumaExperience>,
    /* this will just be the sum of the Exerinces of that type
        // modifer to the event's base_traumatic_stress to get the actual traumatic_stress she experinces
        //  the value in the map is (gradient, y_intercept)
        // actual_stress = gradient * base_stess + y_intercept
        ability_to_handle_emotional_experience_type:
            BTreeMap<TraumaExperienceType, (Number, Number)>,
    */
    // dysregulated_gradient and dysregulated_y_intercept are just placehold. in production
    // dysregulation should be handlesby the emotional_dysregulation module
    dysregulated_gradient: Number,
    dysregulated_y_intercept: Number,
    // Location in the comfort zone (average of how well handled experiences)
    // thi is a phenomenon, a person's experince of the situation,
    location_in_comfort_zone: Number,
}

impl Psych {
    pub fn get_recent_experiences(&self) -> &BTreeMap<TraumaExperienceId, TraumaExperience> {
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
            recent_experiences: BTreeMap::new(),

            location_in_comfort_zone: 0.into(),
            dysregulated_gradient: Number::ONE,
            dysregulated_y_intercept: Number::ZERO,
        }
    }
    fn get_ability_to_deal_with_stress_while_dysregulated(&self) -> (Number, Number) {
        (
            self.dysregulated_gradient.clone(),
            self.dysregulated_y_intercept.clone(),
        )
    }
    fn get_ability_to_handle_emotional_experience_type(
        &self,
        trauma_type: &TraumaExperienceType,
    ) -> (Number, Number) {
        self.recent_experiences
            .iter()
            .filter_map(|(_id, experience)| {
                if &experience.trauma_type == trauma_type {
                    Some((self.dysregulated_gradient, self.dysregulated_y_intercept))
                } else {
                    None
                }
            })
            .fold((Number::ONE, Number::ZERO), |acc, this| {
                (acc.0 + this.0, acc.1 + this.1)
            })
    }

    pub fn calc_trauma_for_bad_experince(&self, base_traumatic_stressfullness: Number) -> Number {
        let (gradient, y_intercept) = self.get_ability_to_deal_with_stress_while_dysregulated();
        let traumatic_stress = gradient * base_traumatic_stressfullness + y_intercept;
        traumatic_stress
    }
    /// Calculate how the inital trauma of the event using the line formula
    pub fn calc_traumatic_stress(
        &self,
        experience_type: &TraumaExperienceType,
        base_traumatic_stressfullness: Number,
    ) -> Number {
        let (gradient, y_intercept) =
            self.get_ability_to_handle_emotional_experience_type(experience_type);
        let traumatic_stress = gradient * base_traumatic_stressfullness + y_intercept;
        traumatic_stress
    }
    pub fn insert_trauma_experience(
        &self,
        _trauma_experince_id: TraumaExperienceId,
        _trauma_experince: TraumaExperience,
    ) -> Option<TraumaExperience> {
        todo!()
    }
    pub fn get_trauma_experience(
        &self,
        _trauma_experince_id: &TraumaExperienceId,
    ) -> Option<&TraumaExperience> {
        todo!()
    }
    pub fn get_trauma_experience_mut(
        &self,
        _trauma_experince_id: &TraumaExperienceId,
    ) -> Option<&mut TraumaExperience> {
        todo!()
    }

    pub fn handle_trauma_event(&mut self, trauma_experince_id: TraumaExperienceId, event: Event) {
        match event {
            Event::Support {
                stressfulness,
                supportiveness,
            } => {
                let Some(experince) = self.get_trauma_experience(&trauma_experince_id) else {
                    return;
                };
                let current_coping_support = experince.get_current_coping_support();

                let (net_support, stressfulness2) = if supportiveness < current_coping_support {
                    (Number::ZERO, supportiveness - stressfulness)
                } else {
                    (supportiveness - stressfulness, stressfulness)
                };

                self.get_trauma_experience_mut(&trauma_experince_id)
                    .expect("we just got this experince so it shoulf still be there")
                    .add_support_event(net_support, stressfulness2);
            }
            Event::Trauma {
                trauma_type,
                base_traumatic_stressfulness,
            } => {
                // Update the number of experiences of this type
                self.num_experiences.add_or_insert(trauma_type.clone(), 1);
                let traumatic_stress =
                    self.calc_traumatic_stress(&trauma_type, base_traumatic_stressfulness);

                let coping_support = match self.get_trauma_experience(&trauma_experince_id) {
                    Some(experience) => experience.get_current_coping_support(),
                    None => Number::ZERO,
                };

                // how much trauma was caused by the event
                let coped_stress = traumatic_stress - coping_support;

                // the effects of that trauma

                // If it was out of her confortzone it was a bad experince
                // use 1 / confortzone to map 0 to inf to inf to 0
                let trauma_tolerance = Number::ONE / self.location_in_comfort_zone.clone();
                let (bad_experience_ka, trauma) = if coped_stress > trauma_tolerance {
                    let trauma = self.calc_trauma_for_bad_experince(coped_stress);
                    (true, trauma)
                } else {
                    (false, coped_stress)
                };

                // effects of her ability to handle that type of experince
                let (new_gradient_delta, new_y_intercept_delta) = if bad_experience_ka {
                    // decrease abiity to handle. the father out of her comfortzone the bigger the decrease use every time
                    let x = traumatic_stress.clone().max(trauma.clone());
                    let trauma_overload = x.clone() - trauma_tolerance.clone();

                    let new_gradient_delta = Number::try_from(
                        (1.0 - ((Into::<f64>::into(&trauma_overload) + 1.0).powi(-1))) * 2.0,
                    )
                    .unwrap();

                    let (_gradient, y_intercept) =
                        self.get_ability_to_handle_emotional_experience_type(&trauma_type);
                    let new_y_intercept_delta =
                        (y_intercept * (x / trauma_tolerance)) - y_intercept;

                    (new_gradient_delta, new_y_intercept_delta)
                } else {
                    // Calculate the increase in her ability to handle this type of experience
                    // the farther away fom her confort zone the less of an improvment in her ability
                    // comfort_zone_factor is 1 is she is at the center of her comfortzone and decreases the father out she is
                    let comfort_zone_factor = Number::ONE
                        / (Number::ONE + (Number::HALF * self.location_in_comfort_zone.clone()));
                    let (gradient, y_intercept) =
                        self.get_ability_to_handle_emotional_experience_type(&trauma_type);
                    // the 4 is fairly arbitray. the biger the value the slowing value grows
                    let base_gradient_improvement_percent =
                        traumatic_stress.clone() / (traumatic_stress.clone() + Number::FOUR);
                    let new_gradient_delta =
                        -(gradient * base_gradient_improvement_percent * comfort_zone_factor);

                    let good_experience_factor = Number::try_from(
                        ((self.get_num_good_experiences(&trauma_type) + 1) as f64).sqrt(),
                    )
                    .unwrap();
                    let perscent_over_tolerance =
                        (traumatic_stress.clone() / trauma_tolerance) - Number::ONE;
                    let new_y_intercept_delta =
                        -(y_intercept * perscent_over_tolerance * good_experience_factor);
                    (new_gradient_delta, new_y_intercept_delta)
                };
                // add the event to the experince
                match self.get_trauma_experience_mut(&trauma_experince_id) {
                    Some(experience) => experience.add_trauma_event(
                        new_gradient_delta,
                        new_y_intercept_delta,
                        trauma,
                        base_traumatic_stressfulness,
                    ),
                    None => {
                        let experience = TraumaExperience::simple_new(
                            new_gradient_delta,
                            new_y_intercept_delta,
                            trauma,
                            base_traumatic_stressfulness,
                            trauma_type,
                        );
                        self.insert_trauma_experience(trauma_experince_id, experience);
                    }
                }
            }
        }
    }
}
