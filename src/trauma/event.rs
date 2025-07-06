use crate::{trauma::TraumaExperienceType, Number};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Event {
        Support{
        stressfulness: Number,
        suppertiveness: Number
    },
    Trauma{
        // Type of trauma experience
        trauma_type: TraumaExperienceType,
        // How traumatic or stressful the Event
        base_traumatic_stressfulness: Number,
    }
}
impl Event {
    pub fn simple_new(trauma_type: TraumaExperienceType, base_traumatic_stressfulness: Number) -> Self {
        Event::Trauma {
            trauma_type,
            base_traumatic_stressfulness,
        }
    }
    pub fn get_trauma_type(&self) -> Option<&TraumaExperienceType> {
        match self{
            Event::Support { stressfulness: _, suppertiveness: _ } => None,
            Event::Trauma { trauma_type, base_traumatic_stressfulness: _ } => Some(trauma_type),
        } 
    }
    pub fn get_base_traumatic_stressfullness(&self) -> Number {
        match self {
            Event::Support { stressfulness, suppertiveness: _ } => stressfulness.clone(),
            Event::Trauma { trauma_type: _ , base_traumatic_stressfulness } => base_traumatic_stressfulness.clone(),
        }
    }
}
