use crate::{trauma::TraumaExperienceType, Number};

// record on her experine of an event
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct TraumaExperience{
    pub experience_type: TraumaExperienceType,
    
}
impl TraumaExperience {
    pub fn get_current_coping_support(&self) -> Number {
        todo!()
    }
    pub fn add_event(
        &mut self,
        _trauma: Number, 
        _base_traumatic_stressfulness: Number,
    ){
        todo!()
    }
    pub fn simple_new(
        _trauma: Number, 
        _base_traumatic_stressfulness: Number,
        _trauma_type: TraumaExperienceType
    ) -> Self{
        todo!()
    }
}
