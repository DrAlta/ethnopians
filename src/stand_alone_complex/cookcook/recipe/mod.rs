use std::collections::BTreeSet;

mod apply_rules;
pub use apply_rules::apply_rules;
mod component;
pub use component::Component;
mod preperation_method;
pub use preperation_method::PreperationMethod;
mod preperation_methods;
pub use preperation_methods::preperation_methods;
mod tag;
pub use tag::Tag;

type TagPropagationRule = (
    BTreeSet<Tag>, 
    BTreeSet<Tag>
);

////////////////////////////////////////////////////////////////////////////////
#[derive(Default, Debug, Clone)]
struct DownPass {

}

#[derive(Default, Debug, Clone)]
struct Characteristics {
    crunchiness: f32,
    moisture: f32,
}

#[derive(Default, Debug, Clone)]
pub struct UpPass {
    children: Vec::<Self>,

    characteristics: Characteristics,
    tags: BTreeSet<Tag>,
}



fn main() {
    // Create a "Soggy Burger" (No Bun/Encapsulation)
    let mut burger = Component::new_assembled(
        PreperationMethod::Assemble,
        vec![
            Component::new_raw(
                Characteristics::default(),
                BTreeSet::from([Tag::CrispyA]
            )), // Fried Patty
            Component::new_raw(
                Characteristics::default(),
                BTreeSet::from([Tag::Wet]
            )), // Sauce
        ]
    );

    // Pass 1
    let pass = burger.synthesize_characteristics();
    let mut pass_maybe = Some(pass);
    // Pass 2
    let mut tag_context = Vec::new();
    burger.compute_scores(&mut pass_maybe, &mut tag_context);

    println!("{pass_maybe:?}");
}
