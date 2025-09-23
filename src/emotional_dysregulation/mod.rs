//!
//! Causes of Emotional Dysregulation
//! There are a few different reasons why someone may develop emotional dysregulation:
//! * Early childhood trauma. These are traumatic events experienced during the early years of a person's life. This is deemed the most critical developmental period in human life.
//! * Child neglect. A form of abuse from caregivers that results in a deprivation of a child’s basic needs, including the failure to provide adequate supervision, health care, clothing, or housing as well as other physical, emotional, social, educational, and safety needs. 
//! * Traumatic brain injury. A brain dysfunction caused by an outside force, usually a violent blow to the head.‌
//! * Chronic low levels of invalidation. This occurs when a person's thoughts and feelings are rejected, ignored, or judged.
//! 
//! Experts suspect that when you experience emotional dysregulation, there is a reduction 
//! in certain neurotransmitters' ability to function as "emotional brakes,'' causing you 
//! to remain in a prolonged “fight or flight” response. When this happens, the pre-frontal cortex 
//! — the part of the brain responsible for emotional regulation — is essentially turned off 
//! during times of heightened stress.
//! 
//! Signs of emotional dysregulation include:
//! * Severe depression
//! * Anxiety
//! * High levels of shame and anger
//! * Self-harm
//! * Excessive substance use
//! * High-risk sexual behaviors
//! * Extreme perfectionism
//! * Conflict in interpersonal relationships
//! * Eating disorder
//! * Suicidal thoughts or attempts

use crate::Number;
pub struct RegulationState {
    buffer: Number,             // accumulated regulatory “currency”
    gradient: Number,           // sensitivity scaling
    intercept: Number,          // baseline offset
    is_dysregulated: bool,
}

impl RegulationState {
    pub fn new(grad: Number, threshold: Number) -> Self {
        let intercept = -(threshold / grad);
        Self {
            buffer: Number::ZERO,
            gradient: grad,
            intercept,
            is_dysregulated: false,
        }
    }

    pub fn get_threshold(&self) -> Number {
         - ( self.intercept / self.gradient)
    }
    
    pub fn get_location(&self) -> Number {
        self.gradient * self.buffer + self.intercept
    }

    pub fn process_regulation(&mut self, regulation_effort: Number) {
        // Add or subtract regulation “currency”
        self.buffer += regulation_effort;
        // Recompute position
        let location = self.get_location();
        // Simple flip
        self.is_dysregulated = location < Number::ZERO;
    }
}
