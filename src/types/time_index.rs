use derive_more::derive::{Add,Sub,Mul,Div,Rem, AddAssign};

#[derive(Debug,Clone, PartialEq, Eq, PartialOrd, Ord,Add,Sub,Mul,Div,Rem, AddAssign)]
pub struct TimeIndex(i64);

impl std::fmt::Display for TimeIndex {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "TimeIndex{}", self.0)
    }
}
