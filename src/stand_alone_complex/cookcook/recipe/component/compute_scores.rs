use std::collections::BTreeSet;

use super::Component;
use super::super::{PreperationMethod, Tag, UpPass};

/// A Depth-First Search that implements "inheritance-style" state passing.
fn dfs<T>(children: &mut Vec<UpPass>, heirloom: &T, fun: fn(&mut UpPass, &T) -> (bool, T)){
    for child in children{
        let (decend, heirloom2) = fun(child, heirloom);

        if decend {
            dfs(&mut child.children, &heirloom2, fun);
        }
    }
}
impl Component {
    pub fn compute_scores(
        &self,
        up_pass: &mut Option<UpPass>,
        branch: &mut Vec<(PreperationMethod,  BTreeSet<Tag>)>
    ) {
        Working_on_this
        /*
        // 1. Check for "Encapsulation" in the branch to handle shielding
        let dried_out_by_deep_frying = {
            let mut dried_out_by_deep_frying = false;
            for seg in branch {
                if seg.0 == PreperationMethod::DeepFry {
                    dried_out_by_deep_frying = true
                    break
                }
                if seg.1.contains(&Tag::Encapsulate) {
                    break
                }
            }
        };
    */
        let Some(pass) = up_pass else {
            return
        };
        match self {
            Component::Assembled { method, ingredients } => {
                if method == &PreperationMethod::DeepFry {
                    for child in &mut pass.children {
                        child.tags.insert(Tag::CrispyA);
                        child.characteristics.crunchiness += 1.0;
                    }
                    dfs(
                        &mut pass.children,
                        &0.25,
                        |x, heirloom|
                        {
                            x.characteristics.moisture *= *heirloom;
                            let new_heirloom = 1.0 - (*heirloom * 0.25);
                            (!x.tags.contains(&Tag::Encapsulate), new_heirloom)
                        }
                    );
                }
            },
            Component::Raw { characteristics, tags } => todo!(),
        }
        todo!("need to owrk out how this work, it goes up the tree passing tages and computing thr base charitieristics to the higher level then it goes back down altering the childresn haracterists based on the currnt level and the path down to it")
    }
}