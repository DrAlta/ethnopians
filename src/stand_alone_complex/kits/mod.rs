use std::collections::{BTreeSet, HashMap};
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Kit<'a, Tag> {
    Primitive(&'a str, &'a BTreeSet<Tag>),
    Constructed(Vec<Self>, BTreeSet<Tag>),
}
impl<'a, Tag> Kit<'a, Tag> {
    /// Returns the tags associated with this kit.
    pub fn tags(&self) -> &BTreeSet<Tag> {
        match self {
            Kit::Primitive(_, tags) => tags,
            Kit::Constructed(_, tags) => tags,
        }
    }
}
impl<'a, Tag: std::fmt::Debug> Kit<'a, Tag> {

    /// Returns a human-readable name for this kit.
    ///
    /// Primitive: returns the item name
    /// Constructed: returns something like "Constructed(A+B)"
    pub fn name(&self) -> String {
        match self {
            Kit::Primitive(name, _) => (*name).to_string(),

            Kit::Constructed(kits, tags) => {

                let mut parts: Vec<String> =
                    tags.iter().map(|t| format!("{:?}", t)).collect();
                parts.sort();

                let kitss: Vec<String> =
                    kits.iter().map(|t| t.name()).collect();
                parts.sort();
                format!("Constructed({}) from [{}]", parts.join("+"), kitss.join(", "))
            }
        }
    }
    pub fn names(&self) -> Vec<&str> {
        match self {
            Kit::Primitive(name, _) => vec![&name],

            Kit::Constructed(kits, _) => {

                kits.iter().map(|t| t.names()).flatten().collect()
            }
        }
    }
}
fn solve_recipe<'a, Tag: Clone + Ord>(
    recipe_reqs: &BTreeSet<BTreeSet<Tag>>,
    available: &[Kit<'a, Tag>],
    recipes: &HashMap<BTreeSet<Tag>, BTreeSet<BTreeSet<Tag>>>,
) -> Option<Vec<Kit<'a, Tag>>> {

    let mut reqs: Vec<&BTreeSet<Tag>> = recipe_reqs.iter().collect();
    reqs.sort();

    let mut used = Vec::new();
    if solve_recipe_backtrack(&reqs, available, recipes, &mut used) {
        Some(used)
    } else {
        None
    }
}
fn solve_recipe_backtrack<'a, Tag: Clone + Ord>(
    reqs: &[&BTreeSet<Tag>],
    available: &[Kit<'a, Tag>],
    recipes: &HashMap<BTreeSet<Tag>, BTreeSet<BTreeSet<Tag>>>,
    used: &mut Vec<Kit<'a, Tag>>,
) -> bool {

    if reqs.is_empty() {
        return true;
    }

    let (first_req, rest) = reqs.split_first().unwrap();

    for (i, kit) in available.iter().enumerate() {
        if kit.tags().is_superset(first_req) {
            // use existing kit
            used.push(kit.clone());

            let mut remaining = available.to_vec();
            remaining.remove(i);

            if solve_recipe_backtrack(rest, &remaining, recipes, used) {
                return true;
            }

            used.pop();
        }
    }

    // If no existing kit works, try constructing one
    if let Some(new_kit) = solve_requirement(first_req, available, recipes) {
        used.push(new_kit.clone());

        let remaining = available.to_vec();
        // constructed kit is unique, so no removal needed

        if solve_recipe_backtrack(rest, &remaining, recipes, used) {
            return true;
        }

        used.pop();
    }

    false
}

pub fn solve_requirement<'a, Tag: Clone + Ord>(
    req: &BTreeSet<Tag>,
    available: &[Kit<'a, Tag>],
    recipes: &HashMap<BTreeSet<Tag>, BTreeSet<BTreeSet<Tag>>>,
) -> Option<Kit<'a, Tag>> {

    // 1. Try to satisfy using an existing primitive or constructed kit
    for kit in available {
        if kit.tags().is_superset(req) {
            return Some(kit.clone());
        }
    }

    // 2. Try to build a kit using a recipe
    for (output_tags, recipe_reqs) in recipes {
        if output_tags.is_superset(req) {
            if let Some(subkits) = solve_recipe(recipe_reqs, available, recipes) {
                return Some(Kit::Constructed(subkits, output_tags.clone()));
            }
        }
    }

    None
}
fn kits_from<'a, Tag>(
    items: &'a HashMap<String, BTreeSet<Tag>>
) -> Vec<Kit<'a, Tag>> {
    items.iter()
        .map(|(name, tags)| Kit::Primitive(name.as_str(), tags))
        .collect()
}
#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
    enum Tag{
        A, 
        B,
        C,
        D,
        E,
        F,
        G,
        H,
    }


    #[macro_export]
    macro_rules! btreeset {
        ( $( $x:expr ),* $(,)? ) => {
            ::std::collections::BTreeSet::from([
                $( $x ),*
            ])
        };
    }

    #[test]
    fn solves_simple_requirement() {
        use Tag::*;

        let mut items = HashMap::new();
        items.insert("Stove".to_string(), btreeset![A, B]);

        let req = btreeset![A];

        let result = solve_requirement(&req, &kits_from(&items), &HashMap::new());
        assert!(result.is_some());
        assert_eq!(result.unwrap().tags(), &btreeset![A, B]);
    }

    #[test]
    fn backtracks_when_item_matches_multiple_requirements() {
        use Tag::*;

        // item1 can satisfy both A and B
        // item2 can satisfy only B
        let mut items = HashMap::new();
        items.insert("item1".to_string(), btreeset![A, B]);
        items.insert("item2".to_string(), btreeset![B]);

        let reqs = btreeset![
            btreeset![A],
            btreeset![B],
        ];

        let result = solve_recipe(&reqs, &kits_from(&items), &HashMap::new());
        assert!(result.is_some());

        let used = result.unwrap();
        let names: Vec<_> = used.iter().map(|k| k.name()).collect();

        // item1 must be used for A, item2 for B
        assert!(names.contains(&"item1".to_string()));
        assert!(names.contains(&"item2".to_string()));
    }

    #[test]
    fn constructs_subkit_via_recipe() {
        use Tag::*;

        // Primitive items
        let mut items = HashMap::new();
        items.insert("wood".to_string(), btreeset![A]);
        items.insert("stone".to_string(), btreeset![B]);

        // Recipe: A + B → C
        let mut recipes = Recipes::new();
        recipes.insert(
            btreeset![C], // output tags
            btreeset![
                btreeset![A],
                btreeset![B],
            ],
        );

        let req = btreeset![C];

        let result = solve_requirement(&req, &kits_from(&items), &recipes);
        assert!(result.is_some());

        let kit = result.unwrap();
        assert!(kit.tags().contains(&C));

        // Ensure it was constructed, not primitive
        match kit {
            Kit::Constructed(subkits, _) => {
                let names: Vec<_> = subkits.iter().map(|k| k.name()).collect();
                assert!(names.contains(&"wood".to_string()));
                assert!(names.contains(&"stone".to_string()));
            }
            _ => panic!("Expected constructed kit"),
        }
    }

    #[test]
    fn avoids_constructing_subkit_that_blocks_other_requirements() {
        use Tag::*;

        // Items:
        // item1: A + B
        // item2: B
        let mut items = HashMap::new();
        items.insert("item1".to_string(), btreeset![A, B]);
        items.insert("item2".to_string(), btreeset![B]);

        // Recipe: A → C
        let mut recipes = Recipes::new();
        recipes.insert(
            btreeset![C],
            btreeset![btreeset![A]],
        );

        // Goal: {C, B}
        // Correct solution:
        // - Use item1 for C (via recipe requiring A)
        // - Use item2 for B
        let reqs = btreeset![
            btreeset![C],
            btreeset![B],
        ];

        let result = solve_recipe(&reqs, &kits_from(&items), &recipes);
        assert!(result.is_some());

        let used = result.unwrap();
        let names: Vec<_> = used.iter().map(|k| k.names()).flatten().collect();
        println!("{names:?}");
        assert!(names.contains(&"item1"));
        assert!(names.contains(&"item2"));
    }

    #[test]
    fn fails_when_no_solution_exists() {
        use Tag::*;

        let mut items = HashMap::new();
        items.insert("wood".to_string(), btreeset![A]);

        let req = btreeset![B]; // impossible

        let result = solve_requirement(&req, &kits_from(&items), &HashMap::new());
        assert!(result.is_none());
    }
    #[test]
    fn alta() {
        use Tag::*;

        // Primitive items
        let mut items = HashMap::new();
        items.insert("wood".to_string(), btreeset![A]);

        // Recipe: A + B → C
        let mut recipes = Recipes::new();
        recipes.insert(
            btreeset![D], // output tags
            btreeset![
                btreeset![C],
            ],
        );
        recipes.insert(
            btreeset![C], // output tags
            btreeset![
                btreeset![B],
            ],
        );
        recipes.insert(
            btreeset![B], // output tags
            btreeset![
                btreeset![A],
            ],
        );

        let req = btreeset![D];

        let result = solve_requirement(&req, &kits_from(&items), &recipes);
        println!("{result:?}");
        assert!(result.is_some());

        let kit = result.unwrap();
        assert!(kit.tags().contains(&C));
        

        // Ensure it was constructed, not primitive
        match kit {
            Kit::Constructed(subkits, _) => {
                let names: Vec<_> = subkits.iter().map(|k| k.name()).collect();
                assert!(names.contains(&"wood".to_string()));
                assert!(names.contains(&"stone".to_string()));
            }
            _ => panic!("Expected constructed kit"),
        }
    }
}