use super::{apply_rules, Component};
use super::super::UpPass;

impl Component {
    /// PASS 1: BOTTOM-UP
    pub fn synthesize_characteristics(&self) -> UpPass {
        match self {
            Component::Assembled { method, ingredients } => {
                let children: Vec<UpPass> = ingredients.iter().map(|child| child.synthesize_characteristics()).collect();
                let (characteristics, tags) = apply_rules(&children, method);

                UpPass{
                    children,
                    characteristics,
                    tags,
                }
            },
            Component::Raw { characteristics, tags } => {
                UpPass{
                    children: Vec::new(),
                    characteristics: characteristics.clone(),
                    tags: tags.clone()
                }
            },
        }
    }
}