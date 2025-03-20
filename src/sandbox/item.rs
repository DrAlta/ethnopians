#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub enum Item {
    Agent,

    Axe,
    Food,
    Stone,
    Stick,
    Wood,

    House,
    Knife,
    Seed,
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
                Item::Knife => "Knife",
                Item::Seed => "Seed",
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
impl TryFrom<String> for Item {
    type Error = ();

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let str: &str = &value;
        str.try_into()
    }
}
impl TryFrom<&str> for Item {
    type Error = ();

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Ok(if value.eq_ignore_ascii_case("Agent") {
            Item::Agent
        } else if value.eq_ignore_ascii_case("Axe") {
            Item::Axe
        } else if value.eq_ignore_ascii_case("Food") {
            Item::Food
        } else if value.eq_ignore_ascii_case("Knife") {
            Item::Knife
        } else if value.eq_ignore_ascii_case("Stone") {
            Item::Stone
        } else if value.eq_ignore_ascii_case("Stick") {
            Item::Stick
        } else if value.eq_ignore_ascii_case("Wood") {
            Item::Wood
        } else if value.eq_ignore_ascii_case("House") {
            Item::House
        } else if value.eq_ignore_ascii_case("Seed") {
            Item::Seed
        } else if value.eq_ignore_ascii_case("Tree") {
            Item::Tree
        } else if value.eq_ignore_ascii_case("Veggie") {
            Item::Veggie
        } else {
            return Err(());
        })
    }
}
