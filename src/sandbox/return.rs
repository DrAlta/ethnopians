#[derive(Debug, PartialEq, PartialOrd, Clone)]
pub enum Return<Command> {
    ActionInvalid(String),
    Commands(Vec<Command>),
}
