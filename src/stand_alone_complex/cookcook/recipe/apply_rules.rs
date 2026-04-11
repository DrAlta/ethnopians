use std::collections::BTreeSet;

use crate::stand_alone_complex::cookcook::recipe::{Characteristics, PreperationMethod, Tag, UpPass, preperation_methods};

pub fn apply_rules(children: &Vec<UpPass>, method: &PreperationMethod) -> (Characteristics, BTreeSet<Tag>) {
    let init = (
        Characteristics{ crunchiness: 0.0, moisture: 0.0 },
        BTreeSet::new(),
    );
    let rules =preperation_methods().remove(method).unwrap_or_default();
    children.iter()
        .fold(
            init,
            |mut acc: (Characteristics, BTreeSet<Tag>), child|
            {
                acc.0.crunchiness += child.characteristics.crunchiness;
                acc.0.moisture += child.characteristics.moisture;
                for rule in &rules {
                    if child.tags.is_superset(&rule.0) {
                        for tag in &rule.1 {
                            acc.1.insert(tag.clone());
                        }
                    }

                };
                acc
            }
        )
}
