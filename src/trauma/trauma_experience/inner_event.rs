use crate::Number;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct InnerEvent {
    pub trauma: Number,
    pub base_traumatic_stressfulness: Number,
}
