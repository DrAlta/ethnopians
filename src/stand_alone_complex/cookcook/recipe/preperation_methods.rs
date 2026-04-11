use std::collections::{BTreeSet, HashMap};

use super::{PreperationMethod, Tag, TagPropagationRule};

pub fn preperation_methods() -> HashMap<PreperationMethod, Vec<TagPropagationRule>>{
    let mut ret = HashMap::new();
    let citrus: TagPropagationRule = 
        (
            BTreeSet::from([Tag::Citrus]),
            BTreeSet::from([Tag::Citrus]),
        );
    let moisture: TagPropagationRule = 
        (
            BTreeSet::from([Tag::Wet]),
            BTreeSet::from([Tag::Wet]),
        );
    let meat_chop: TagPropagationRule =
        (
            BTreeSet::from([Tag::Meat]),
            BTreeSet::from([Tag::Wet]),
        );
    ret.insert(
        PreperationMethod::Chop,
        vec![
            citrus,
            moisture,
            meat_chop,
        ]
    );

    ret
}

