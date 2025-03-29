#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Treeturn {
    Success,
    Failure,
    Running(Vec<String>),
}
