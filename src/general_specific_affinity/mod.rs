mod character;
pub use character::Character;
mod event;
pub use event::Event;
mod preference;
pub use preference::Preference;
mod response;
pub use response::Response;
mod rule;
pub use rule::Rule;
use std::collections::BTreeSet;

mod determine_preference;
pub use determine_preference::determine_preference;
mod is_more_specific;
pub use is_more_specific::is_more_specific;





pub fn main() {
    let general_disapprove_fighting = Rule {
        // Empty event tags for a general rule
        event_tags: BTreeSet::new(),
        response_tags: ["fighting".to_string()].into(),
        preference: Preference::Disapprove,
    };

    let specific_approve_righteous_fighting = Rule {
        // Specific event tags that must be matched
        event_tags: ["help".to_string(), "community".to_string()].into(),
        // Specific response tags that must be matched
        response_tags: ["righteous".to_string(), "fighting".to_string()].into(),
        preference: Preference::Approve,
    };

    let character = Character {
        name: "The Paladin".to_string(),
        rules: vec![general_disapprove_fighting, specific_approve_righteous_fighting],
    };

    // Scenario that matches the specific rule
    let event_tags = ["help".to_string(), "community".to_string(), "raid".to_string()]
        .into();
    let response_tags = ["fighting".to_string(), "righteous".to_string()]
        .into();

    let event = Event { tags: event_tags };
    let response = Response { tags: response_tags };

    match determine_preference(&character, &event, &response) {
        Some(preference) => println!("The Paladin's preference in this specific context: {:?}", preference),
        None => println!("The Paladin has no strong preference in this context."),
    }

    // Scenario that matches only the general rule
    let general_event_tags = ["artistic".to_string(), "show".to_string()]
        .into();
    let general_response_tags = ["fighting".to_string(), "disruptive".to_string()]
        .into();

    let general_event = Event { tags: general_event_tags };
    let general_response = Response { tags: general_response_tags };
    
    println!("\n--- Another scenario: an artistic show ---");
    match determine_preference(&character, &general_event, &general_response) {
        Some(preference) => println!("The Paladin's preference: {:?}", preference),
        None => println!("The Paladin has no strong preference."),
    }
}
