#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Treeturn {
    Success,
    Failure,
    Running(Vec<String>),
}
