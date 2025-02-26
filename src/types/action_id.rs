#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ActionId(usize);

impl ActionId {
    pub const USE_OBJECT: Self = ActionId(0);
}

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
        match self {
            &Self::USE_OBJECT => {
                write!(f, "Action:Use_Object")

            }
            _ => {
                write!(f, "Action{}", self.0)
            }
        }
    }
}
