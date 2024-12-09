#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub enum Item {
    Agent,

    Axe,
    Food,
    Stone,
    Stick,
    Wood,

    House,
    Tree,
    Veggie,
}

impl std::fmt::Display for Item {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Item::Agent => "Agent",
                Item::Axe => "Axe",
                Item::Food => "Food",
                Item::Stone => "Stone",
                Item::Stick => "Stick",
                Item::Wood => "Wood",
                Item::House => "House",
                Item::Tree => "Tree",
                Item::Veggie => "Veggie",
            }
        )
    }
}
