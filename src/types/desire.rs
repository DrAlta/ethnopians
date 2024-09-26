use derive_more::derive::{Add, AddAssign, Div, Mul, Rem, Sub};

#[derive(
    Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord, Add, Sub, Mul, Div, Rem, AddAssign,
)]
pub struct Desire(i64);

impl std::fmt::Display for Desire {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Desire{}", self.0)
    }
}
