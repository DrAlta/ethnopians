#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ActionId(usize);

type Id = ActionId;

impl Into<usize> for &Id {
    fn into(self) -> usize {
        self.0
    }
}

impl Into<usize> for Id {
    fn into(self) -> usize {
        self.0
    }
}

impl From<usize> for Id {
    fn from(value: usize) -> Self {
        Self(value)
    }
}

impl std::fmt::Display for Id {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Action{}", self.0)
    }
}
