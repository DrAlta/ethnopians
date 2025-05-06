use crate::sandbox::ItemClass;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum InpulseId {
    Act1,
    Act2,
    Act3,

    GoTo,
    Plant,
    Take,
    Use,
    UseOn,
    EatClass(ItemClass),
}
