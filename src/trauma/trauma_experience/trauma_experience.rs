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
    // should we make spressing inner_events for supportive and traumatice events?
    // curectly we just consider any event with a negatice trauma asa supportive event.
    pub inner_events: Vec<InnerEvent>,

    pub coping_threshold: Number,
}
impl TraumaExperience {
    pub fn add_support_event(&mut self, net_support: Number, stress: Number) {
        if net_support < Number::ZERO {
            return;
        };
        self.coping_threshold = Number::ZERO.max(net_support + self.coping_threshold);
        self.inner_events.push(InnerEvent {
            trauma: -net_support,
            base_traumatic_stressfulness: stress,
        });
    }
    pub fn get_current_coping_support(&self) -> Number {
        let support_count = self
            .inner_events
            .iter()
            .filter_map(
                |&InnerEvent {
                     trauma,
                     base_traumatic_stressfulness: _,
                 }| {
                    if trauma < Number::ZERO {
                        Some(())
                    } else {
                        None
                    }
                },
            )
            .count();
        self.coping_threshold * Into::<Number>::into(support_count)
    }
    pub fn add_trauma_event(
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
            coping_threshold: Number::ZERO,
        }
    }
}
