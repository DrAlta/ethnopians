use crate::{
    trauma::{trauma_experience::InnerEvent, TraumaExperienceType},
    Number,
};

// record on her experine of an event
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct TraumaExperience {
    pub trauma_type: TraumaExperienceType,

    pub ability_to_handle_gradient: Number,
    pub ability_to_handle_y_intercept: Number,
    pub inner_events: Vec<InnerEvent>,
}
impl TraumaExperience {
    pub fn get_current_coping_support(&self) -> Number {
        todo!()
    }
    pub fn add_event(
        &mut self,
        new_gradient_delta: Number,
        new_y_intercept_delta: Number,
        trauma: Number,
        base_traumatic_stressfulness: Number,
    ) {
        self.ability_to_handle_gradient =
            (self.ability_to_handle_gradient + new_gradient_delta) * Number::HALF;
        self.ability_to_handle_y_intercept =
            (self.ability_to_handle_y_intercept + new_y_intercept_delta) * Number::HALF;
        self.inner_events.push(InnerEvent {
            trauma,
            base_traumatic_stressfulness,
        });
    }
    pub fn simple_new(
        new_gradient_delta: Number,
        new_y_intercept_delta: Number,
        trauma: Number,
        base_traumatic_stressfulness: Number,
        trauma_type: TraumaExperienceType,
    ) -> Self {
        Self {
            trauma_type,
            ability_to_handle_gradient: new_gradient_delta,
            ability_to_handle_y_intercept: new_y_intercept_delta,
            inner_events: vec![InnerEvent {
                trauma,
                base_traumatic_stressfulness,
            }],
        }
    }
}
