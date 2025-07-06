//! start 08-17-2024
//! when a traumaic eperine is handles succes fuly we'll set the slop of the line use to get how tramix an enevet if based on the base tramaicness of the event to be closer to 0 then adjust the xintersept so that it is closer
//!
//! when a theumatic event isn't coped with successfully then we find the slop of a line with the same intercept as the old one but goes throu a point hights then adjust ithe ix intercept so that the event would have the same final traumaticness
//!
//! ?maby avg the old and new x-intercept based on how much trauma they have?
//! ```
//! // the weight on the new x_intercept is the percentage of her thrematic experinces have turned out that way
//! // we add 1 to the demon so we don't devide by zero then we add 0.5 to the numer so that with out any experinces it's 50/50
//! pub fn calc_weights_for_updated_x_intercept(was_handled_successfully:bool, number_of_unsuccessfully_handled_trauma: f64, number_of_successfully_handled_trauma: f64) {
//!     let delta_base = (number_of_successfully_handled_trauma + 0.5) / ((number_of_successfully_handled_trauma +  number_of_unsuccessfully_handled_trauma) + 1.0);
//!     let (weight_of_old_x_intercept, weight_of_new_x_intercept ) = if was_handled_successfully {
//!         (
//!             1.0 - delta_base,
//!             delta_base,
//!         )
//!     } else {
//!         (
//!             delta_base,
//!             1.0 - delta_base,
//!         )
//!     };
//! }
//! ```
//! end 08-17-2024

//! events have base_trauma_stress
//! base_trauma_stress is uses as a x value into a line equation that is the girls ability to handle trauma
//! this gives the effective_traumatic_stress of the event
//! comf_zone is how much traumatic_stress she can handle
//! trauma  is the traumatic_stress left over after she handled it `trauma = effective_traumatic_stress - comfort_zone`
//! trauma of 1 is is enough to be a bad experince when at loc+in_comf_zone 0.0
//! trauma cause the slope to decrease
//! calc new line with that slope so that effective_traumatic_stress produced by the same base_traumatic_stress the same
//!
//! is effe_traumatic_stress == comf_zone then use the new line as the new ablit ot handle trauma
//!
//! let b =   effective_traumatic_stress / comfurt_zone
//! final_x_intercept = ((intermediat_x-intecept - old_x_interceop) * b) + old_x_intercept
mod event;
pub use event::Event;
mod psych;
pub use psych::Psych;
mod trauma_experience_type;
pub use trauma_experience_type::TraumaExperienceType;
mod experience;
pub use experience::TraumaExperience;


pub type TraumaExperienceId = u64;

/*
#[allow(dead_code)]
fn test() {
    let mut pawn = psych::Psych::new();

    let event = Event::simple_new(TraumaExperienceType::Humiliation, Number::ONE);
    let stage1 = pawn.handle_experience_stage_1(event);
    let _x = pawn.handle_experience_stage_2(stage1, Number::ONE);
    pout!("{:#?}", pawn.get_recent_experiences());
}
*/