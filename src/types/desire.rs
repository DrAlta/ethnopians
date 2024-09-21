use derive_more::derive::{Add,Sub,Mul,Div,Rem, AddAssign};

#[derive(Debug,Clone, PartialEq, Eq, PartialOrd, Ord,Add,Sub,Mul,Div,Rem, AddAssign)]
pub struct Desire(i64);

impl std::fmt::Display for Desire {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Desire{}", self.0)
    }
}
