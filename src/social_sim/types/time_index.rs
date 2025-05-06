use derive_more::derive::{Add, AddAssign, Div, Mul, Rem, Sub};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Add, Sub, Mul, Div, Rem, AddAssign)]
pub struct TimeIndex(i64);

impl std::fmt::Display for TimeIndex {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "TimeIndex{}", self.0)
    }
}
