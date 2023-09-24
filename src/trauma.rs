use fraction::GenericFraction;
type Fraction = GenericFraction::<u32>;


mod psych;

// Define an enum to represent emotional experience types
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone)]
pub enum TraumaExperienceType {
    Humiliation
    // Add more types as needed
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone)]
pub struct Event {
    // Type of trauma experience
    trauma_type: TraumaExperienceType,
    // How traumatic or stressful the Event
    base_traumatic_stress: Fraction,
}
impl Event {
    pub fn simple_new(trauma_type: TraumaExperienceType, base_traumatic_stress: Fraction) -> Self {
        Self{trauma_type, base_traumatic_stress}
    }
    pub fn get_trauma_type(&self) -> &TraumaExperienceType {
        &self.trauma_type
    }
    pub fn get_base_traumatic_stress(&self) -> Fraction {
        self.base_traumatic_stress.clone()
    }
    
}

// record on her experine of an event
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone)]
pub struct TraumaExperience {
    // Type of emotional experience
    experience_type: TraumaExperienceType,
    // trauma  is the traumatic_stress left over after she handled it
    //trauma of 1 is is enough to be a bad experince when at loc+in_comf_zone 0.0

    trauma: Fraction,
    // traumatic_stress is the straess before she handled it
    traumatic_stress: Fraction,
}
impl TraumaExperience {
    pub fn simple_new(experience_type: TraumaExperienceType, trauma: Fraction, traumatic_stress: Fraction) -> Self {
        Self{experience_type, trauma, traumatic_stress}
    }
    
}

#[allow(dead_code)]
fn test(){
    let mut pawn = psych::Psych::new();

    let event = Event::simple_new(TraumaExperienceType::Humiliation, Fraction::from(1));
    let stage1 = pawn.handle_experience_stage_1(event);
    let _x = pawn.handle_experience_stage_2(stage1, Fraction::from(1));
    println!("{:#?}", pawn.get_recent_experiences());
}

