#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Return<Command> {
    ActionInvalid(String),
    Commands(Vec<Command>),
}
