use crate::Number;


#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct MassageSkill{
    pub arousal_base: Number,
    pub arousal_slope: Number,

    pub apprehension_base: Number,
    pub apprehension_slope: Number,

    pub knowledge_base: Number,
    pub knowledge_slope: Number,

    pub skill_base: Number,
    pub skill_slope: Number,
}