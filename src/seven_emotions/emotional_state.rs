use crate::Number;


// We’ll use a simple model where each emotional process is represented as a value between 0.0 and 1.0.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct EmotionalState {
    // these are how 'active' the system is 0.0 if inactive. 1.0 is active at full strangth
    /// seeking is increased by novel stimuli, you're walking downa trai land hear one unsual noise you might walk on, but hear a buch and you're curiosity and peeked and to stop to investagate it.
    seeking: Number,
    /// rage is caused by precives  something as taking away your abiliti to adapt. limiting your future state in  'future state maximization' theory themr, to really it's only the future states you where reasoning about and more specificly the ones you where actually considering, if you belaive your would never climb to the top of the bell tower with a sniper riffle and start picking people off, having that future state removed isn't going to stimulate RAGE
    rage: Number,
    /// fear is the response to the detection of threats to our survival and set into motion a biobehavioral response that would facilitate fighting or fleeing
    fear: Number,
    ///
    lust: Number,
    /// care_activation = (perceived_need_the_suject_has_for_protection * kawaii_of_subject) +  (perceived_need_the_suject_has_for_protection * emational_attachment_to_subject)
    care: Number,
    /// panic is caused by the perceived social seperation
    panic: Number,
    play: Number,
}

impl EmotionalState {
    /// Convert the current EmotionalState into a PAD (Pleasure, Arousal, Dominance) tuple.
    /// All output values are normalized to range between -1.0 and 1.0.
    pub fn to_pad(&self) -> (Number, Number, Number) {
        // Pleasure computation:
        //   Positive signals = seeking, lust, care, play.
        //   Negative signals = rage, fear, panic.
        // We calculate the average of positives and negatives, then subtract.
        let pos_avg = (self.seeking + self.lust + self.care + self.play) / Number::FOUR;
        let neg_avg = (self.rage + self.fear + self.panic) / Number::THREE;
        let mut pleasure = pos_avg - neg_avg;
        // Clamp to [-1, 1] just in case.
        pleasure = pleasure.clamp(Number::NEG_ONE, Number::ONE);

        // Arousal computation:
        // We use the average level of all emotions as a proxy for arousal.
        // When all values are at 0.5, the result will be neutral.
        let total =
            self.seeking + self.rage + self.fear + self.lust + self.care + self.panic + self.play;
        let arousal_raw = total / Number::SEVEN;
        // Map raw arousal from [0, 1] to [-1, 1]:
        let mut arousal = Number::TWO * arousal_raw - Number::ONE;
        arousal = arousal.clamp(Number::NEG_ONE, Number::ONE);

        // Dominance computation:
        // Empowering signals: rage, lust, seeking.
        // Submissive signals: fear, panic.
        let pos_dom = (self.rage + self.lust + self.seeking) / Number::THREE;
        let neg_dom = (self.fear + self.panic) / Number::TWO;
        let mut dominance = pos_dom - neg_dom;
        dominance = dominance.clamp(Number::NEG_ONE, Number::ONE);

        (pleasure, arousal, dominance)
    }
}
