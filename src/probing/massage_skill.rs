use crate::Number;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct MassageSkill{
    pub arousal_arousal_base: Number,
    pub arousal_arousal_slope: Number,

    pub arousal_apprehension_base: Number,
    pub arousal_apprehension_slope: Number,

    pub arousal_knowledge_base: Number,
    pub arousal_knowledge_slope: Number,

    pub arousal_skill_base: Number,
    pub arousal_skill_slope: Number,


    pub apprehension_arousal_base: Number,
    pub apprehension_arousal_slope: Number,

    pub apprehension_apprehension_base: Number,
    pub apprehension_apprehension_slope: Number,

    pub apprehension_knowledge_base: Number,
    pub apprehension_knowledge_slope: Number,

    pub apprehension_skill_base: Number,
    pub apprehension_skill_slope: Number,
}
