use std::collections::BTreeSet;

use crate::stand_alone_complex::cookcook::recipe::{Characteristics, PreperationMethod, Tag, UpPass, apply_rules};

mod compute_scores;
mod synthesize_characteristics;
pub enum Component {
    Assembled{
        method: PreperationMethod,
//        tags: Vec<Tag>,
        ingredients: Vec<Component>,
    },
    Raw{
        characteristics: Characteristics,
        tags: BTreeSet<Tag>,
    }
}

impl Component {
    pub fn new_assembled(method: PreperationMethod, ingredients: Vec<Component>) -> Self {
        Component::Assembled { method, ingredients } 
    }
    pub fn new_raw(
        characteristics: Characteristics,
        tags: BTreeSet<Tag>,
    ) -> Self {
        Component::Raw { characteristics, tags }
    }


    /*
    /// PASS 2: TOP-DOWN
    /// We pass a "branch" context down the tree.
    /// Why: A leaf ingredient (e.g., CrispyA) only becomes "Crunchy" if a parent was "Fry".
    fn compute_scores(
        &mut self, 
        branch_methods: &mut Vec<PreperationMethod>, branch_tags: &mut Vec<BTreeSet<Tag>>) {
        // 1. Check for "Encapsulation" in the branch to handle shielding
        let is_shielded = branch_tags.iter().any(|tags| tags.contains(&Tag::Encapsulate));
        
        let mut local_crunch = 0.0;
        let mut local_moisture = 0.0;

        // 2. Score calculation based on context
        for tag in &self.all_tags {
            match tag {
                Tag::CrispyA => {
                    // Only grant crunch if a parent fried it
                    if branch_methods.contains(&PreperationMethod::Fry) || matches!(self.method, PreperationMethod::Fry) {
                        local_crunch += 50.0;
                    }
                }
                Tag::Wet => {
                    local_moisture += 30.0;
                }
                _ => {}
            }
        }

        // 3. Apply the "Soggy" penalty if moisture exists and we aren't shielded
        if local_moisture > 0.0 && !is_shielded {
            local_crunch *= 0.2; 
        }

        self.final_scores.crunchiness = local_crunch;
        self.final_scores.moisture = local_moisture;

        // 4. Recurse: Push this component's state onto the branch for the children
        branch_methods.push(self.method);
        branch_tags.push(self.all_tags.clone());

        for child in &mut self.ingredients {
            child.compute_scores(branch_methods, branch_tags);
        }

        // 5. Cleanup: Pop state so we don't pollute sibling branches (Standard Backtracking)
        branch_methods.pop();
        branch_tags.pop();
    }
    */
}
