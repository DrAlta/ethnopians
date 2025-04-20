use super::Skillette;


#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct MassageSkill{
    pub arousal: Skillette,

    pub apprehension: Skillette,
}
