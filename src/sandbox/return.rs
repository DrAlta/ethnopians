#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Return<Command> {
    ActionInvalid(String),
    Commands(Vec<Command>),
}
