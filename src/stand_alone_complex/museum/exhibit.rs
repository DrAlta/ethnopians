use crate::Number;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Exhibit {
    pub buzz: Number,
    pub attractiveness: Number,
}
